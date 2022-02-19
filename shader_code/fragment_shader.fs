#version 330 core

in vec3 export_color;
in vec2 output_texture_coord;
in float export_fog;

out vec4 frag_color;

uniform sampler2D texture_sampler;

//the basic no frills fast graphics mode - now with depth fog
void main()
{    
    vec4 pre_mix = texture( texture_sampler, output_texture_coord) * vec4(export_color, 1.0);

    float fog_factor = clamp(export_fog, 0.0, 1.0);
    
    frag_color = vec4(mix(vec3(135.0 / 255.0, 206.0 / 255.0, 235.0 / 255.0), pre_mix.xyz, fog_factor), 1.0);
}