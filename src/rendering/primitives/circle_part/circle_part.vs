#version 410 core

in vec2 radial_dim;
in vec2 angular_dim;
in vec3 pos;
in vec4 colour;

out vec2 radial_dim_vs;
out vec2 angular_dim_vs;
out vec3 pos_vs;
out vec4 colour_vs;

void main()
{
  radial_dim_vs = radial_dim;
  angular_dim_vs = angular_dim;
  pos_vs = pos;
  colour_vs = colour;
}
