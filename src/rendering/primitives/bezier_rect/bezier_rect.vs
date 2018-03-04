#version 410 core

in vec2 c0;
in vec2 c1;
in vec2 c2;
in float height;
in vec3 pos;
in vec4 color;

out vec2 c0_vs;
out vec2 c1_vs;
out vec2 c2_vs;
out float height_vs;
out vec3 pos_vs;
out vec4 color_vs;

void main()
{
  c0_vs = c0;
  c1_vs = c1;
  c2_vs = c2;
  height_vs = height;
  pos_vs = pos;
  color_vs = color;
}
