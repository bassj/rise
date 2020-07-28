#version 450

layout(location=0) in vec3 a_position;

layout(set=0, binding=0)
uniform CameraUniform {
    mat4 u_view_mat;
    mat4 u_proj_mat;
};

void main() {
    gl_Position = u_proj_mat * vec4(a_position, 1.0);
} 