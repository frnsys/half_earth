// https://www.shadertoy.com/view/3tjfzy
#define _dither(c,u,d) floor(fract(dot(vec2(131,312),u)/vec3(103,71,97))*.375-.1875+c*d)/d

vec3 simple_dither(vec3 c) {
    return _dither(c, gl_FragCoord.xy, 5.);
}
