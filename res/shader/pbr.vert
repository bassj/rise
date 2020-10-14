#version 450

layout(location=0) in vec3 a_position;
layout(location=1) in vec3 a_normal;
layout(location=2) in vec2 a_texcoord;

layout(location=0) out vec3 v_normal;
layout(location=1) out vec2 v_texcoord;
layout(location=2) out vec3 v_worldpos;
layout(location=3) out vec3 v_campos;

layout(set=0, binding=0)
uniform CameraUniform {
    mat4 u_view_mat;
    mat4 u_proj_mat;
};

void main() { 
    mat4 vp_mat =  u_proj_mat * u_view_mat;
    
    v_normal = a_normal;
    v_texcoord = a_texcoord;

    vec4 pos = vp_mat * vec4(a_position, 1.0);

    v_worldpos = a_position;
    v_campos = vec3(0, 0, 5);

    gl_Position = pos;
} 