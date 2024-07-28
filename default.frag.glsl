#version 330 core

out vec4 out_Color;

uniform vec3 u_Color;

void main() {
    out_Color = vec4(u_Color, 1.0);
}