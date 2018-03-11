#version 410 core

in vec2 radial_dim;
in vec2 angular_dim;
in vec3 pos;
in vec4 color;
in uint fixed_pos;

out vec2 radial_dim_vs;
out vec2 angular_dim_vs;
out vec3 pos_vs;
out vec4 color_vs;
out uint fixed_pos_vs;

void main()
{
  radial_dim_vs = radial_dim;
  angular_dim_vs = angular_dim;
  pos_vs = pos;
  color_vs = color;
  fixed_pos_vs = fixed_pos;
}
