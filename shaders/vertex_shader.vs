#version 330 core

layout (location = 0) in vec3 pos;
layout (location = 1) in vec2 tex;

out vec2 sharedTex;

uniform mat4 transformMatrix;

void main() {
    gl_Position = transformMatrix * vec4(pos, 1.0);
    sharedTex = tex;
}