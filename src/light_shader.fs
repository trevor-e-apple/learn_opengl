#version 330 core
in vec3 fragWorldPos;
in vec3 normal;
in vec2 TexCoords;

out vec4 FragColor;

struct Material {
   sampler2D diffuse;
   sampler2D specular;
   float shininess;
};

struct Light {
   vec3 position;

   vec3 ambient;
   vec3 diffuse;
   vec3 specular;
};

uniform vec3 viewPos;
uniform Material material;
uniform Light light;

void main()
{
   vec3 unitNormal = normalize(normal);
   vec3 lightDir = normalize(light.position - fragWorldPos);

   // ambient
   vec3 ambient = light.ambient * texture(material.diffuse, TexCoords).rgb;

   // diffuse
   float diff = max(dot(unitNormal, lightDir), 0.0);
   vec3 diffuse = light.diffuse * diff * texture(material.diffuse, TexCoords).rgb;

   // specular
   vec3 viewDir = normalize(viewPos - fragWorldPos);
   vec3 reflectDir = reflect(-lightDir, unitNormal);
   float spec = pow(max(dot(viewDir, reflectDir), 0.0), material.shininess);
   vec3 specular = light.specular * (spec * texture(material.specular, TexCoords).rgb);

   vec3 result = ambient + diffuse + specular;
   FragColor = vec4(result, 1.0);
}