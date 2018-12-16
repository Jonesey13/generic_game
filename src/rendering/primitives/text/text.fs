#version 410 core
uniform sampler2D tex;
in vec2 tex_pos;
in vec4 color_ges;
in vec4 rect_pos;
out vec4 f_color;

void main() {
  if (texture(tex, tex_pos).r + texture(tex, tex_pos).g + texture(tex, tex_pos).b > 0.1)
  {
    f_color = color_ges * vec4(1.0, 1.0, 1.0, texture(tex, tex_pos).r);
  } else {
    discard;
  }
}
