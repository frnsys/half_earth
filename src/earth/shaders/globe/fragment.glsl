varying float height;
varying vec2 vertexUV;
varying vec3 vertexNormal;
varying vec3 vertexPosition;
uniform sampler2D heightmap;
uniform sampler2D shadows;
uniform sampler2D satTexture;
uniform sampler2D biomesTexture;
uniform vec3 screenRes;
uniform float time;

#include "./lib/bit_dither.glsl"
#include "./lib/blender_noise.glsl"
/* #include "./lib/detect_edges.glsl" */

// For cloud layer
const float detail = 12.160;
const float roughness = 0.625;
const float distortion = 1.050;


void main() {
    // Heightmap coloring
    // https://www.shadertoy.com/view/XsKBzt
    /* vec3 heightColor; */
    /* if (height > 0.5) { */
    /*     heightColor = vec3(1. - 0.1 * clamp(floor(1.5 * height), 0., 4.)); */
    /* } else { */
    /*     heightColor = vec3(1.0, 1.0, 1.0); */
    /* } */
    /* vec3 heightEdges = detect_edges(heightColor); */
    /* vec3 heightEdges = detect_edges(texture2D(heightmap, vertexUV).rgb); */

    // Color based on biomes
    vec3 color = texture2D(biomesTexture, vertexUV).rgb;
    /* vec3 color = scale2x(biomesTexture, vertexUV).rgb; */

    // Add satellite texture
    /* color *= texture2D(satTexture, vertexUV).r * 1.5; */

    // Add light edge highlighting
    vec3 shadows = texture2D(shadows, vertexUV).rgb;
    /* vec3 edges = max(min(heightEdges, detect_edges(color)), 0.75); */

    // Atmospheric glow
    /* float intensity = 1.05 - dot(vertexNormal, vec3(0.0, 0.0, 1.0)); */
    float intensity = 1.05 - dot(vertexNormal, vec3(0.0, 0.0, 1.0));
    vec3 atmosphere = vec3(0.3, 0.3, 0.3) * pow(intensity, 3.);
    float sphereShadow = clamp(dot(vertexNormal, vec3(-2.0, 3.0, 3.0)), -0.5, 1.0);

    // A kind of digital display effect?
    // Probably too distracting
    vec2 checker = step(0.5, fract(vertexUV.xy * 1000.));
    float multiplier = (1. + (checker.x * checker.y * 0.5));
    /* gl_FragColor = vec4(bit_dither((shadows * color * edges) + atmosphere), 1.0) * multiplier; */
    /* gl_FragColor = vec4(bit_dither((shadows * color * edges) + atmosphere), 1.0); */
    /* gl_FragColor = vec4(bit_dither((shadows * color * edges) + atmosphere * sphereShadow), 1.0); */
    /* gl_FragColor = vec4(bit_dither((shadows * color) + atmosphere * sphereShadow) * multiplier, 1.0); */
    /* gl_FragColor = vec4(bit_dither((shadows * color * edges) + atmosphere * sphereShadow) * multiplier, 1.0); */
    /* gl_FragColor = vec4(bit_dither((shadows * color) + atmosphere * sphereShadow) * multiplier, 1.0); */

    vec4 base_color = vec4(bit_dither((shadows * color) + atmosphere * sphereShadow) * multiplier, 1.0);

    // Apply cloud layer
    float scale=0.5 + sin(time/100000.)/2.;
    float n = blender_noise(vertexPosition + time/100000., scale, detail, roughness, distortion);
    gl_FragColor = n < 0.58 ? base_color : base_color + vec4(n);
}
