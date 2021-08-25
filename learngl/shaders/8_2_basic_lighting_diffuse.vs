#version 330 core
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec3 aNormal;

out vec3 Normal;
out vec3 FragPosition;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

void main() {
  gl_Position = projection * view * model * vec4(aPos, 1.0);

  Normal = aNormal;
  // We do all lighting calculations in world space, so we want the vertex
  // position in world space first.
  FragPosition = vec3(model * vec4(aPos, 1.0));
}
