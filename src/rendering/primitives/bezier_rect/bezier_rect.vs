#version 410 core

in vec2 c0;
in vec2 c1;
in vec2 c2;
in vec2 vert_dir;
in float width;
in vec2 pos;
in vec4 color;

out vec2 c0_vs;
out vec2 c1_vs;
out vec2 c2_vs;
out vec2 vert_dir_vs;
out float width_vs;
out vec2 pos_vs;
out vec4 color_vs;

void main()
{
  c0_vs = c0;
  c1_vs = c1;
  c2_vs = c2;
  vert_dir_vs = vert_dir;
  width_vs = width;
  pos_vs = pos;
  color_vs = color;
}
