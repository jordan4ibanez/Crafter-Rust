#version 330 core

in  vec3 exColor;
in  vec2 outTexCoord;

out vec4 fragColor;

uniform sampler2D texture_sampler;

//the basic no frills fast graphics mode
void main()
{
    fragColor = texture( texture_sampler, outTexCoord) * vec4(exColor, 1.0);
}