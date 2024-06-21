// https://www.shadertoy.com/view/WstXR8

mat4 bayerIndex = mat4(
    vec4(00.0/16.0, 12.0/16.0, 03.0/16.0, 15.0/16.0),
    vec4(08.0/16.0, 04.0/16.0, 11.0/16.0, 07.0/16.0),
    vec4(02.0/16.0, 14.0/16.0, 01.0/16.0, 13.0/16.0),
    vec4(10.0/16.0, 06.0/16.0, 09.0/16.0, 05.0/16.0));

vec3 bayer_dither(vec3 color) {
    float bayerValue = bayerIndex[int(gl_FragCoord.x) % 4][int(gl_FragCoord.y) % 4];
    color = pow(color,vec3(2.2)) - 0.004; // gamma correction
    return vec3(step(bayerValue, color.r),
                step(bayerValue, color.g),
                step(bayerValue, color.b));
}
