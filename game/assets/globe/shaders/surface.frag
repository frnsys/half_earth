layout (location = 0) out vec4 outColor;

uniform sampler2D uShadows;
uniform sampler2D uSatTexture;
uniform sampler2D uBiomesTexture;

in vec2 uvs;
in vec3 nor;

// https://www.shadertoy.com/view/3tjfzy
#define _dither(c,u,d) floor(fract(dot(vec2(131.0,312.0),u)/vec3(103.0,71.0,97.0)) * 0.375 - 0.1875 + c*d) / d

vec3 simple_dither(vec3 c) {
    return _dither(c, gl_FragCoord.xy, 5.0);
}

void main() {
    // Color based on biomes
    vec3 color = texture(uBiomesTexture, uvs).rgb;

    // Add satellite texture
    color *= texture(uSatTexture, uvs).r * 1.5;

    // Add precomputed shadows
    vec3 shadowRGB = texture(uShadows, uvs).rgb;

    // Atmospheric glow and sphere shadow, to give more depth
    float intensity = 1.2 - dot(nor, vec3(0.0, 0.0, 1.0));
    vec3 atmosphere = vec3(0.1) * pow(intensity, 2.0);
    float sphereShadow = clamp(dot(nor, vec3(-2.0, 3.0, 3.0)), -0.25, 1.0);
    vec3 shading = atmosphere * sphereShadow * 0.25;

    outColor = vec4(simple_dither((shadowRGB * color) + shading), 1.0);
}
