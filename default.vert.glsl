#version 330 core

layout(location = 0) in vec3 a_Pos;
layout(location = 1) in vec2 a_UV;

out vec2 f_UV;

void main() {
    f_UV = a_UV;
    gl_Position = vec4(a_Pos, 1.0);
}