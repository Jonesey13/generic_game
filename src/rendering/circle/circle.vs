#version 410 core

in float radius;
in vec3 pos;
in vec4 color;

out float radius_vs;
out vec3 pos_vs;
out vec4 color_vs;

void main()
{
  radius_vs = radius;
  pos_vs = pos;
  color_vs = color;
}
