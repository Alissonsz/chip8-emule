#version 330 core
in vec2 texCoords;
out vec4 FragColor;

uniform sampler2D myTexture;

void main() {
    float color = texture(myTexture, texCoords).r;
    FragColor = vec4(color, color, color, 1.0f);
}