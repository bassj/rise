#version 450

layout(location=0) in vec3 a_position;
layout(location=1) in vec3 a_normal;
layout(location=2) in vec2 a_texcoord;

layout(location=0) out vec3 v_normal;

layout(set=0, binding=0)
uniform CameraUniform {
    mat4 u_view_mat;
    mat4 u_proj_mat;
};

void main() {
    mat4 vp_mat =  u_proj_mat * u_view_mat;
    
    v_normal = a_normal;
    
    gl_Position = vp_mat * vec4(a_position, 1.0);
} 