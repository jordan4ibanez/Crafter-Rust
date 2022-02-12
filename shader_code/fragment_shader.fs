#version 330 core

out vec4 FragColor;

uniform vec4 color;
uniform float light;

void main() {
    FragColor = color;
}