#version 410 core

in vec2 c0;
in vec2 c1;
in vec2 c2;
in vec2 vert_dir;
in float bezier_width;
in vec2 pos;
in vec4 color;
in float logic_length_left;
in float logic_length_right;
in float logic_height_left;
in float logic_height_right;
in float length;
in float height;
in vec2 sub_pos;

out vec2 c0_vs;
out vec2 c1_vs;
out vec2 c2_vs;
out vec2 vert_dir_vs;
out float bezier_width_vs;
out vec2 pos_vs;
out vec4 color_vs;
out float logic_length_left_vs;
out float logic_length_right_vs;
out float logic_height_left_vs;
out float logic_height_right_vs;
out float length_vs;
out float height_vs;
out vec2 sub_pos_vs;

void main()
{
  c0_vs = c0;
  c1_vs = c1;
  c2_vs = c2;
  vert_dir_vs = vert_dir;
  bezier_width_vs = bezier_width;
  pos_vs = pos;
  color_vs = color;
  logic_length_left_vs = logic_length_left;
  logic_length_right_vs = logic_length_right;
  logic_height_left_vs = logic_height_left;
  logic_height_right_vs = logic_height_right;
  length_vs = length;
  height_vs = height;
  sub_pos_vs = sub_pos;
}
