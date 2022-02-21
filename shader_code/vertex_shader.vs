#version 330 core

layout (location = 0) in vec3 position;
layout (location = 1) in vec3 input_color;
layout (location = 2) in vec2 texture_coord;

out vec3 export_color;
out vec2 output_texture_coord;
// out float export_fog;

uniform mat4 model_matrix;
uniform mat4 projection_matrix;
// uniform float game_render_distance;


void main()
{
  
    gl_Position = projection_matrix * model_matrix * vec4(position, 1.0);

    export_color = input_color;
    output_texture_coord = texture_coord;


    // float distance = length(vec3(gl_Position.x, gl_Position.y, gl_Position.z));

    // float density = 2.0 / (game_render_distance);
    // float fog = 1.0 / exp( (distance * density) * (distance * density));

    // export_fog = fog;
}