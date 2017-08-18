#version 410 core

in vec2 corner1;
in vec2 corner2;
in vec2 center;
in mat2 rot;
in vec3 pos;
in vec4 color;
in uint fixed_pos;

out vec2 corner1_vs;
out vec2 corner2_vs;
out vec2 center_vs;
out mat2 rot_vs;
out vec3 pos_vs;
out vec4 color_vs;
out uint fixed_vs;

void main()
{
  corner1_vs = corner1;
  corner2_vs = corner2;
  center_vs = center;
  rot_vs = rot;
  pos_vs = pos;
  color_vs = color;
  fixed_vs = fixed_pos;
}
