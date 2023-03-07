#version 330 core

uniform sampler2D u_texture;

in vec2 v_texture_position;

layout(location = 0) out vec4 color;

void main() {
    color = texture(u_texture, v_texture_position);
}
