#version 330 core

layout (location = 0) in vec3 pos;
layout (location = 1) in vec3 tex;

uniform mat4 transformMatrix;

void main() {
    gl_Position = transformMatrix * vec4(pos.x, pos.y, pos.z, 1.0);
}