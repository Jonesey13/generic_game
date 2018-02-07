#version 410 core

uniform sampler2DArray  tex;

in vec3 texture_corner_ges;

out vec4 value;

void main()
{
  value = texture(tex, texture_corner_ges);
}
