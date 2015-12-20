#version 410 core

in float length;
in float height;
in mat2 rot;
in vec3 pos;

out float length_vs;
out float height_vs;
out mat2 rot_vs;
out vec3 pos_vs;

void main()
{
  length_vs = length;
  height_vs = height;
  rot_vs = rot;
  pos_vs = pos;
}
