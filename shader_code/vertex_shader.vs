#version 330 core

layout (location = 0) in vec3 position;
layout (location = 1) in vec3 inColor;
layout (location = 2) in vec2 texCoord;

out vec3 exColor;
out vec2 outTexCoord;


uniform vec4 pos;

//uniform mat4 modelViewMatrix;
//uniform mat4 projectionMatrix;

void main()
{
    gl_Position = vec4(position, 1.0) + pos;
    exColor = inColor;
    outTexCoord = texCoord;
}