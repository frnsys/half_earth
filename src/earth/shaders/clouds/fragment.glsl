uniform float time;
varying vec3 vertexNormal;
varying vec3 vertexPosition;

#include "../lib/clouds.glsl"

void main() {
    // Apply cloud layer
    float scale = 0.25 + max(sin(time/10000.), 0.75)/5.;
    float n = clouds(vertexPosition + time/100000., scale);
    gl_FragColor = n < 0.58 ? vec4(0.0) : vec4(n+0.35);
}
