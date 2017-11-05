#version 410 core
uniform sampler2D tex;
in vec2 tex_pos;
in vec4 colour_ges;
in vec4 rect_pos;
out vec4 f_colour;

void main() {
  f_colour = colour_ges * vec4(1.0, 1.0, 1.0, texture(tex, tex_pos).r);
}