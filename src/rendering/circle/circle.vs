#version 410 core

in float radius;
in vec3 pos;

out float radius_vs;
out vec3 pos_vs;

void main()
{
  radius_vs = radius;
  pos_vs = pos;
}
