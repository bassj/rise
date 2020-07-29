#version 450

layout(location=0) in vec3 a_position;

layout(set=0, binding=0)
uniform CameraUniform {
    mat4 u_view_mat;
    mat4 u_proj_mat;
};

void main() {

    mat4 vp_mat =  u_proj_mat * u_view_mat;

    gl_Position = vp_mat * vec4(a_position, 1.0);
} 