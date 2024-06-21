/* Adapted from <https://www.shadertoy.com/view/XsKBzt> */
varying vec2 vertexUV;
varying vec3 vertexNormal;
varying vec3 vertexPosition;
varying float height;
uniform sampler2D heightmap;

const mat2 m = mat2( 1.6,  1.2, -1.2,  1.6 );

vec2 hash( vec2 p ) {
    p = vec2(dot(p,vec2(127.1,311.7)), dot(p,vec2(269.5,183.3)));
    return -1.0 + 2.0*fract(sin(p)*43758.5453123);
}

float noise( in vec2 p ) {
    const float K1 = 0.366025404; // (sqrt(3)-1)/2;
    const float K2 = 0.211324865; // (3-sqrt(3))/6;
    vec2 i = floor(p + (p.x+p.y)*K1);
    vec2 a = p - i + (i.x+i.y)*K2;
    vec2 o = (a.x>a.y) ? vec2(1.0,0.0) : vec2(0.0,1.0); //vec2 of = 0.5 + 0.5*vec2(sign(a.x-a.y), sign(a.y-a.x));
    vec2 b = a - o + K2;
    vec2 c = a - 1.0 + 2.0*K2;
    vec3 h = max(0.5-vec3(dot(a,a), dot(b,b), dot(c,c) ), 0.0 );
    vec3 n = h*h*h*h*vec3( dot(a,hash(i+0.0)), dot(b,hash(i+o)), dot(c,hash(i+1.0)));
    return dot(n, vec3(70.0));
}

float fbm(vec2 n) {
    float total = 0.0, amplitude = 0.1;
    for (int i = 0; i < 7; i++) {
        total += noise(n) * amplitude;
        n = m * n;
        amplitude *= 0.4;
    }
    return total;
}

void main() {
    vertexUV = uv;
    vertexNormal = normalize(normalMatrix * normal);

    // Calculate height based on heightmap
    vec4 c = texture2D(heightmap, uv);
    float land = 1. - clamp(c.b - c.r + 3. * fbm(32. * uv), 0., 1.);
    height = land + 3. * c.r/0.31;

    // TODO heightmap is causing seams to appear
    // Scale displacement and displace along normal
    float displacement = (height - 0.5) * 0.2;
    vec4 pos = vec4(position + displacement * normal, 1.0);
    vertexPosition = pos.xyz;

    gl_Position = projectionMatrix * modelViewMatrix * pos;
    /* gl_Position = projectionMatrix * modelViewMatrix * vec4(position, 1.0); */
}
