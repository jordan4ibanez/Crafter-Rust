#version 330 core

in vec3 export_color;
in vec2 output_texture_coord;
// in float output_render_distance;

out vec4 frag_color;

uniform sampler2D texture_sampler;

//the basic no frills fast graphics mode
void main()
{
    frag_color = texture( texture_sampler, output_texture_coord) * vec4(export_color, 1.0);
}