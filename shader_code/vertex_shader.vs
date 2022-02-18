#version 330 core

layout (location = 0) in vec3 position;
layout (location = 1) in vec3 input_color;
layout (location = 2) in vec2 texture_coord;

out vec3 export_color;
out vec2 output_texture_coord;
// out float output_render_distance;

uniform mat4 model_view_matrix;
uniform mat4 projection_matrix;

// uniform float render_distance;

void main()
{
    gl_Position = projection_matrix * model_view_matrix * vec4(position, 1.0);
    export_color = input_color;
    output_texture_coord = texture_coord;
    // output_render_distance = render_distance;
}