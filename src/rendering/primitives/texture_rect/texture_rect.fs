#version 410 core

uniform sampler2DArray  tex;

in vec3 texture_corner_ges;

out vec4 value;

void main()
{
  vec4 texture_value = texture(tex, texture_corner_ges);
  
  if (texture_value.a < 0.5) { discard; }

  value = texture_value;
}
