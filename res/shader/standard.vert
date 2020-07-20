#version 330 core
layout (location = 0) in vec2 aPos;
layout (location = 1) in vec2 aTexCoord;

uniform mat4 projectionMatrix;
uniform mat4 modelMatrix;
uniform mat4 viewMatrix;

out vec2 texCoord;

void main() {
    mat4 mvpMatrix = (projectionMatrix * viewMatrix * modelMatrix);

    gl_Position = mvpMatrix * vec4(aPos, 0.1, 1.0);
    texCoord = aTexCoord;
}
