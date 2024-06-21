// https://www.shadertoy.com/view/Xtd3W7

vec3 detect_edges(vec3 color) {
    float gray = length(color.rgb);
    return 1. - vec3(step(0.06, length(vec2(dFdx(gray), dFdy(gray)))));
}
