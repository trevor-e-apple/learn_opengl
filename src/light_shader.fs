#version 330 core
in vec3 fragWorldPos;
in vec3 normal;

out vec4 FragColor;

uniform vec3 objectColor;
uniform vec3 lightColor;
uniform vec3 lightPos;

void main()
{
   float ambientStrength = 0.1;
   vec3 ambient = ambientStrength * lightColor;

   vec3 unitNormal = normalize(normal);
   vec3 lightDir = normalize(lightPos - fragWorldPos);

   float diff = max(dot(unitNormal, lightDir), 0.0);
   vec3 diffuse = diff * lightColor;

   vec3 result = (ambient + diffuse) * objectColor;
   FragColor = vec4(result, 1.0);
}