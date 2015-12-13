#version 400 Core

in float width;
in float height;
in vec3 position;
in mat2 rotation;

out float width_vs;
out float height_vs;
out vec3 position_vs;
out mat2 rotation_vs;

void main() {
  width_vs = width;
  height_vs = height;
  rotation_vs = rotation;
}
