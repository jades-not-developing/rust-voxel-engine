#version 330 core

layout(location = 0) in vec3 a_Pos;
layout(location = 1) in vec2 a_UV;

uniform mat4 u_Transform;
uniform mat4 u_Projection;

out vec2 f_UV;

void main() {
    f_UV = a_UV;
    gl_Position = u_Projection * u_Transform * vec4(a_Pos, 1.0);
    // gl_Position = u_Transform * vec4(a_Pos, 1.0);
}