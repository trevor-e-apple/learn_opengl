#version 330 core
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec3 aColor;
layout (location = 2) in vec2 aTexCoord;

out vec3 ourColor;
out vec2 TexCoord;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;


void main()
{
   vec4 transformed = vec4(aPos, 1.0) * model * view * projection;
   gl_Position = transformed;

   // mat4 test = mat4(
   //    vec4(1.0, 0.0, 0.0, 0.0),
   //    vec4(0.0, 1.0, 0.0, 0.0),
   //    vec4(0.0, 0.0, 1.0, -3.0),
   //    vec4(0.0, 0.0, 0.0, 1.0)
   // );
   // vec4 test2 = vec4(aPos, 1.0);
   // vec4 transformed = test2 * test;
   // gl_Position = vec4(aPos, 1.0);

   // float dot_product = view[2][0] * aPos.x + view[2][1] * aPos.y + view[2][2] * aPos.z + view[2][3] * 1.0;
   // if (transformed.w > 0.6 && transformed.w < 0.61) {
   //    ourColor = vec3(1.0, 0.0, 0.0);
   // }
   // else {
   //    ourColor = vec3(0.0, 0.0, 1.0);
   // }
   TexCoord = aTexCoord;
}
