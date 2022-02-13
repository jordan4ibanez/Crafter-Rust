#version 330 core
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec3 aColor;
layout (location = 2) in vec2 aTexCoord;

out vec3 ourColor;
out vec2 TexCoord;
out vec3 color_modifier;

// uniforms are modifiers for meshes
uniform vec3 pos;
uniform vec3 color;

void main()
{

	gl_Position = vec4(aPos + pos, 1.0);

	ourColor = aColor;
	color_modifier = color;
	TexCoord = vec2(aTexCoord.x, aTexCoord.y);

}