const float PIXEL_FACTOR = 840.; // Lower num - bigger pixels (this will be the screen width)
const float COLOR_FACTOR = 8.;   // Higher num - higher colors quality

const mat4 ditherTable = mat4(
    -4.0, 0.0, -3.0, 1.0,
    2.0, -2.0, 3.0, -1.0,
    -3.0, 1.0, -4.0, 0.0,
    3.0, -1.0, 2.0, -2.0
);

vec3 bit_dither(vec3 color) {
    // Reduce pixels
    vec2 size = PIXEL_FACTOR * screenRes.xy/screenRes.x;
    vec2 coor = floor( gl_FragCoord.xy/screenRes.xy * size) ;
    vec2 uv = coor / size;
   	// Get source color

    // Dither
    color += ditherTable[int( coor.x ) % 4][int( coor.y ) % 4] * 0.005; // last number is dithering strength

    // Reduce colors
    color = floor(color * COLOR_FACTOR) / COLOR_FACTOR;

    return color;
}
