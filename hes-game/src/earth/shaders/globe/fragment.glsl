varying float height;
varying vec2 vertexUV;
varying vec3 vertexNormal;
varying vec3 vertexPosition;
uniform sampler2D heightmap;
uniform sampler2D shadows;
uniform sampler2D satTexture;
uniform sampler2D biomesTexture;
uniform vec3 screenRes;

#include "../lib/simple_dither.glsl"

void main() {
    // Color based on biomes
    vec3 color = texture2D(biomesTexture, vertexUV).rgb;

    // Add satellite texture
    color *= texture2D(satTexture, vertexUV).r * 1.5;

    // Add precomputed shadows
    vec3 shadows = texture2D(shadows, vertexUV).rgb;

    // Atmospheric glow and sphere shadow, to give more depth
    float intensity = 1.2 - dot(vertexNormal, vec3(0.0, 0.0, 1.0));
    vec3 atmosphere = vec3(0.3, 0.3, 0.3) * pow(intensity, 3.);
    float sphereShadow = clamp(dot(vertexNormal, vec3(-2.0, 3.0, 3.0)), -0.5, 1.0);

    gl_FragColor = vec4(simple_dither((shadows * color) + atmosphere * sphereShadow), 1.0);
}
