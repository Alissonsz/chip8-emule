#version 330 core
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec2 aTexPos;

out vec2 texCoords;

uniform mat4 projection;
uniform mat4 view;

void main() {
    texCoords = aTexPos; 
    gl_Position = projection * view * vec4(aPos.x, aPos.y, aPos.z, 1.0);
}