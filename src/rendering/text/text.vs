#version 410 core

in float length;
in float height;
in vec2 local_position;
in vec2 position;
in vec2 tex_coords_min;
in vec2 tex_coords_max;
in vec2 scale;
in mat2 transform;
in vec4 colour;

out float length_vs;
out float height_vs;
out vec2 local_position_vs;
out vec2 position_vs;
out vec2 tex_coords_min_vs;
out vec2 tex_coords_max_vs;
out vec2 scale_vs;
out mat2 transform_vs;
out vec4 colour_vs;

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
}
