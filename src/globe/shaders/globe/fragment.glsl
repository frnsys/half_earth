varying float height;
varying vec2 vertexUV;
varying vec3 vertexNormal;
varying vec3 vertexPosition;
uniform sampler2D heightmap;
uniform sampler2D satTexture;
uniform sampler2D labelsTexture;
uniform vec3 screenRes;

#include "./lib/bit_dither.glsl"
#include "./lib/detect_edges.glsl"
#include "./lib/hexasphere.glsl"


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
    vec3 heightEdges = detect_edges(texture2D(heightmap, vertexUV).rgb);

    // Color based on labels
    vec3 color = texture2D(labelsTexture, vertexUV).rgb;

    // Add satellite texture
    color *= texture2D(satTexture, vertexUV).r;

    // Add light edge highlighting
    /* vec3 edges = max(min(heightEdges, detect_edges(color)), 0.6); */
    vec3 edges = vec3(0.9);

    // Atmospheric glow
    float intensity = 1.05 - dot(vertexNormal, vec3(0.0, 0.0, 1.0));
    vec3 atmosphere = vec3(0.3, 0.3, 0.3) * pow(intensity, 3.);

    // A kind of digital display effect?
    // Probably too distracting
    /* vec2 checker = step(0.5, fract(vertexUV.xy * 1000.)); */
    /* float multiplier = (1. + (checker.x * checker.y * 0.5)); */
    /* gl_FragColor = vec4(bit_dither((edges * color) + atmosphere), 1.0) * multiplier; */
    /* gl_FragColor = vec4(bit_dither((edges * color * max(hexasphere(), 0.8)) + atmosphere), 1.0); */
    gl_FragColor = vec4(bit_dither((edges * color) + atmosphere), 1.0);
}
