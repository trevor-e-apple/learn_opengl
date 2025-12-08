#version 330 core
in vec3 fragWorldPos;
in vec3 normal;

out vec4 FragColor;

uniform vec3 objectColor;
uniform vec3 lightColor;
uniform vec3 lightPos;
uniform vec3 viewPos;

void main()
{
   float ambientStrength = 0.1;
   vec3 ambient = ambientStrength * lightColor;

   vec3 unitNormal = normalize(normal);
   vec3 lightDir = normalize(lightPos - fragWorldPos);

   float diff = max(dot(unitNormal, lightDir), 0.0);
   vec3 diffuse = diff * lightColor;

   float specularStrength = 0.5;
   vec3 viewDir = normalize(viewPos - fragWorldPos);
   vec3 reflectDir = reflect(-lightDir, unitNormal);
   float spec = pow(max(dot(viewDir, reflectDir), 0.0), 32);
   vec3 specular = specularStrength * spec * lightColor;

   vec3 result = (ambient + diffuse + specular) * objectColor;
   FragColor = vec4(result, 1.0);
}