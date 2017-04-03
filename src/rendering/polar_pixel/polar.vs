/*
Standard Polar Vertex Shader
 */

#version 410 core
in vec2 radial;
in vec2 angle;
in vec4 color;

uniform float radial_shift;
uniform float rotation_angle;

out vec2 radial_vertex;
out vec2 angle_vertex;
out vec4 color_vertex;

void main()
{
  radial_vertex = vec2(max(radial.x - radial_shift, 0.0f), max(radial.y - radial_shift, 0.0f));
  angle_vertex = vec2(angle.x - rotation_angle, angle.y - rotation_angle);
  color_vertex = color;

  gl_Position = vec4(0.0f, 0.0f, 0.0f, 1.0f);
}
