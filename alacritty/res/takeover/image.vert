#version 330 core

uniform vec2 u_img_scale;

layout(location = 0) in vec2 a_position;
layout(location = 1) in vec2 a_texture_position;

out vec2 v_texture_position;

void main() {
    gl_Position = vec4(
        a_position.x * u_img_scale.x,
        a_position.y * u_img_scale.y,
        0.0,
        1.0
    );
    v_texture_position = a_texture_position;
}
