#version 410 core

in float length;
in float height;
in mat2 rot;
in vec3 pos;
in vec3 texture_corner;
in vec2 texture_dim;
in uint fixed_pos;

out float length_vs;
out float height_vs;
out mat2 rot_vs;
out vec3 pos_vs;
out vec3 texture_corner_vs;
out vec2 texture_dim_vs;
out uint fixed_vs;

void main()
{
  length_vs = length;
  height_vs = height;
  rot_vs = rot;
  pos_vs = pos;
  texture_corner_vs = texture_corner;
  texture_dim_vs = texture_dim;
  fixed_vs = fixed_pos;
}
