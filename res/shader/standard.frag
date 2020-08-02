#version 450

layout(location=0) in vec3 v_normal;

layout(location=0) out vec4 f_color;

void main() {
    //light comes from front top left
    vec3 light_dir = normalize(vec3(-1.0, 1.0, 1.0));

    float ambient_strength = 0.1;
    float diffuse_strength = max(dot(v_normal, light_dir), 0.0);

    vec3 base_color = vec3(1.0, 1.0, 1.0);
    vec3 light_color = vec3(1.0, 1.0, 1.0);


    vec3 ambient_color = base_color * ambient_strength;
    vec3 diffuse_color = light_color * diffuse_strength;

    vec3 final_color = (ambient_color + diffuse_color) * base_color;

    f_color = vec4(final_color, 1.0); 
}