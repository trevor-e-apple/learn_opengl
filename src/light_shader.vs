#version 330 core
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec3 aNormal;

out vec3 fragWorldPos;
out vec3 normal;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;


void main()
{
   vec4 transformed = projection * view * model * vec4(aPos, 1.0);
   gl_Position = transformed;

   fragWorldPos = vec3(model * vec4(aPos, 1.0));
   normal = vec3(model * vec4(aNormal, 1.0));
}
