#version 410 core

in float length;
in float height;
in vec2 local_position;
in vec3 position;
in vec2 tex_coords_min;
in vec2 tex_coords_max;
in vec2 scale;
in mat2 transform;
in vec4 colour;
in uint fixed_pos;

out float length_vs;
out float height_vs;
out vec2 local_position_vs;
out vec3 position_vs;
out vec2 tex_coords_min_vs;
out vec2 tex_coords_max_vs;
out vec2 scale_vs;
out mat2 transform_vs;
out vec4 colour_vs;
out uint fixed_pos_vs;

void main() {
  length_vs = length;
  height_vs = height;
  local_position_vs = local_position;
  position_vs = position;
  tex_coords_min_vs = tex_coords_min;
  tex_coords_max_vs = tex_coords_max;
  scale_vs = scale;
  transform_vs = transform;
  colour_vs = colour;
  fixed_pos_vs = fixed_pos;
}
