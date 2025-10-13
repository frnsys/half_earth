layout (location = 0) out vec4 outColor;

uniform sampler2D shadows;
uniform sampler2D satTexture;
uniform sampler2D biomesTexture;

in vec2 uvs;
in vec3 nor;

// https://www.shadertoy.com/view/3tjfzy
#define _dither(c,u,d) floor(fract(dot(vec2(131,312),u)/vec3(103,71,97))*.375-.1875+c*d)/d

vec3 simple_dither(vec3 c) {
    return _dither(c, gl_FragCoord.xy, 5.);
}

void main() {
    // Color based on biomes
    vec3 color = texture2D(biomesTexture, uvs).rgb;

    // Add satellite texture
    color *= texture2D(satTexture, uvs).r * 1.5;

    // Add precomputed shadows
    vec3 shadows = texture2D(shadows, uvs).rgb;

    // Atmospheric glow and sphere shadow, to give more depth
    float intensity = 1.2 - dot(nor, vec3(0.0, 0.0, 1.0));
    vec3 atmosphere = vec3(0.1, 0.1, 0.1) * pow(intensity, 2.);
    float sphereShadow = clamp(dot(nor, vec3(-2.0, 3.0, 3.0)), -0.25, 1.0);
    vec3 shading = atmosphere * sphereShadow * 0.25;

    outColor = vec4(simple_dither((shadows * color) + shading), 1.0);
}
