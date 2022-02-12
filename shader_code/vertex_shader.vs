#version 330 core

layout (location = 0) in vec3 aPos;

uniform vec4 pos;

void main() {
    gl_Position = vec4(aPos.x + pos.x, aPos.y + pos.y, aPos.z + pos.z, 1.0 + pos.w);
}