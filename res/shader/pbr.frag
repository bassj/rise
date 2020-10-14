#version 450

layout(location=0) in vec3 v_normal;
layout(location=1) in vec2 v_texcoord;
layout(location=2) in vec3 v_worldpos;
layout(location=3) in vec3 v_campos;

layout(location=0) out vec4 f_color;

layout(set = 1, binding = 0) uniform texture2D t_diffuse;
layout(set = 1, binding = 1) uniform sampler s_diffuse;

//MATHS STUFF
const float PI = 3.14159265359;

vec3 fresnelSchlick(float cosTheta, vec3 F0)
{
    return F0 + (1.0 - F0) * pow(1.0 - cosTheta, 5.0);
} 

float DistributionGGX(vec3 N, vec3 H, float roughness)
{
    float a      = roughness*roughness;
    float a2     = a*a;
    float NdotH  = max(dot(N, H), 0.0);
    float NdotH2 = NdotH*NdotH;
	
    float num   = a2;
    float denom = (NdotH2 * (a2 - 1.0) + 1.0);
    denom = PI * denom * denom;
	
    return num / denom;
}

float GeometrySchlickGGX(float NdotV, float roughness)
{
    float r = (roughness + 1.0);
    float k = (r*r) / 8.0;

    float num   = NdotV;
    float denom = NdotV * (1.0 - k) + k;
	
    return num / denom;
}

float GeometrySmith(vec3 N, vec3 V, vec3 L, float roughness)
{
    float NdotV = max(dot(N, V), 0.0);
    float NdotL = max(dot(N, L), 0.0);
    float ggx2  = GeometrySchlickGGX(NdotV, roughness);
    float ggx1  = GeometrySchlickGGX(NdotL, roughness);
	
    return ggx1 * ggx2;
}

//END MATHS STUFF

void main() {
    //light comes from front top left
    vec3 light_worldpos = vec3(2, 2, 2);
    vec3 light_color = vec3(1, 1, 1);


    vec3 albedo = vec3(0.8196, 0.6039, 0.1059);
    float metallic = 0.1;
    float roughness = 0.0;
    float ao = 0.6;

    vec3 N = normalize(v_normal);
    vec3 V = normalize(v_campos - v_worldpos);

    vec3 F0 = vec3(0.04); 
    F0 = mix(F0, albedo, metallic);

    // Calculate light from point light.

    vec3 Lo = vec3(0);

    vec3 L = normalize(light_worldpos - v_worldpos);
    vec3 H = normalize(V + L);

    float distance = length(light_worldpos - v_worldpos);
    float attenuation = 1.0 / distance * distance;
    vec3 radiance = light_color * attenuation;

    float NDF = DistributionGGX(N, H, roughness);        
    float G   = GeometrySmith(N, V, L, roughness);      
    vec3 F    = fresnelSchlick(max(dot(H, V), 0.0), F0);
        
    vec3 kS = F;
    vec3 kD = vec3(1.0) - kS;
    kD *= 1.0 - metallic;	  
        
    vec3 numerator    = NDF * G * F;
    float denominator = 4.0 * max(dot(N, V), 0.0) * max(dot(N, L), 0.0);
    vec3 specular     = numerator / max(denominator, 0.001);  
            
    // add to outgoing radiance Lo
    float NdotL = max(dot(N, L), 0.0);                
    Lo += (kD * albedo / PI + specular) * radiance * NdotL; 

    vec3 ambient = vec3(0.03) * albedo * ao;
    vec3 color = ambient + Lo;

    color = color / (color + vec3(1.0));
    color = pow(color, vec3(1.0/2.2));  

    f_color = vec4(color, 1.0); 
}