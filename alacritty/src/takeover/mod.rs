use std::path::PathBuf;
use winit::event::{WindowEvent, KeyboardInput, ModifiersState, VirtualKeyCode};
use winit::dpi::PhysicalSize;
use crate::display::Display;
use log::error;

mod shader;
mod quad;
mod renderer;
mod texture;

use renderer::ImageRenderer;

trait TakeoverRenderer {
    fn render(&self);

    fn resize(&mut self, size: &PhysicalSize<u32>);
}

/// Takeover functionality. Signals being ready to rock.
pub struct Takeover {
    pub active: bool,
    /// Trait object that can render something to the screen.
    renderer: Option<Box<dyn TakeoverRenderer>>,
}

impl std::fmt::Debug for Takeover {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Takeover").field("active", &self.active).finish()
    }
}

impl Default for Takeover {
    fn default() -> Self {
        Self {
            active: false,
            renderer: None,
        }
    }
}

impl Takeover {
    pub fn render(&mut self) {
        debug_assert!(self.active);
        debug_assert!(self.renderer.is_some());
        self.renderer.as_ref().unwrap().render();
    }

    pub fn update(&mut self, event: TakeoverEvent, display: &Display) {
        if self.active {
            return;
        }

        // TODO: In the future should not discard renderer instance.
        //       Same renderer types could reuse existing resources.
        let _ = self.renderer.take();

        match event {
            TakeoverEvent::Image(path) => {
                match ImageRenderer::create(path, display) {
                    Ok(r) => {
                        self.renderer.replace(Box::new(r));
                        self.active = true;
                    },
                    Err(msg) => {
                        error!("[Takeover]: {msg}");
                    },
                }
            },
        }
    }

    pub fn resize(&mut self, size: &PhysicalSize<u32>) {
        self.renderer.as_mut().unwrap().resize(size);
    }

    #[inline]
    fn exit(&mut self, display: &Display) {
        self.active = false;
        display.window.request_redraw();
    }

    fn key_input(&mut self, input: &KeyboardInput, mods: &ModifiersState, display: &Display) {
        // For now the only key to handle is ESC (C-[)
        match input.virtual_keycode {
            Some(VirtualKeyCode::Escape) => self.exit(display),
            Some(VirtualKeyCode::LBracket) => {
                if mods.ctrl() || mods.logo() {
                    self.exit(display);
                }
            }
            _ => (),
        }
    }

    pub fn handle_window_event(&mut self, event: &WindowEvent<'_>, display: &Display, modifiers: &ModifiersState) {
        debug_assert!(self.active);

        match event {
            WindowEvent::Resized(size) => {
                self.resize(size);
            },
            WindowEvent::KeyboardInput { input, is_synthetic: false, .. } => {
                self.key_input(&input, modifiers, display);
            }
            _ => (),
        }
    }
}

/// Event types handled by Takeover.
#[non_exhaustive]
#[derive(Debug, Clone)]
pub enum TakeoverEvent {
    /// Displays image available under provided full path.
    /// Path must point to data on the host file system.
    ///
    /// Expected message format: "image:{absolute path}".
    Image(PathBuf),
}

// Helpers for converting input string.
impl<'a> TryFrom<&'a str> for TakeoverEvent {
    type Error = ();

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        if let Some(path_str) = value.strip_prefix("image:") {
            let path = PathBuf::from(path_str);

            // Make sure the path is absolute, the file exists.
            // TODO: And the file points to an image (maybe check extensions).
            if !path.is_absolute() || !path.is_file() {
                return Err(());
            }

            return Ok(Self::Image(path));
        }

        return Err(());
    }
}
