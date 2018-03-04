#version 410 core

in vec2 c0;
in vec2 c1;
in vec2 c2;
in float branch_height;
in vec3 branch_pos;
in float radius;
in float branch_logical_length;
in float branch_logical_height_left;
in float branch_logical_height_right;
in vec2 logical_pos;
in vec4 color;

out vec2 c0_vs;
out vec2 c1_vs;
out vec2 c2_vs;
out float branch_height_vs;
out vec3 branch_pos_vs;
out float radius_vs;
out float branch_logical_length_vs;
out float branch_logical_height_left_vs;
out float branch_logical_height_right_vs;
out vec2 logical_pos_vs;
out vec4 color_vs;

void main()
{
  c0_vs = c0;
  c1_vs = c1;
  c2_vs = c2;
  branch_height_vs = branch_height;
  branch_pos_vs = branch_pos;
  radius_vs = radius;
  branch_logical_length_vs = branch_logical_length;
  branch_logical_height_left_vs = branch_logical_height_left;
  branch_logical_height_right_vs = branch_logical_height_right;
  logical_pos_vs = logical_pos;
  color_vs = color;
}
