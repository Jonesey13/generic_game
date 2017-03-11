#version 410 core
uniform float aspect_ratio;

in float length;
in float height;
in mat2 rot;
in vec3 pos;
in vec4 color;

out float length_vs;
out float height_vs;
out mat2 rot_vs;
out vec3 pos_vs;
out vec4 color_vs;

void main()
{
  length_vs = length / aspect_ratio;
  height_vs = height;
  rot_vs = rot;
  pos_vs = vec3(pos.x / aspect_ratio, pos.yz);
  color_vs = color;
}
