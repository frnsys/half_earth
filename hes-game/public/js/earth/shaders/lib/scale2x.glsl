// https://www.shadertoy.com/view/4l2SRz

float vec4Equal(vec4 a, vec4 b)
{
    const vec4 epsilon = vec4(1.0/255.0); //should be considered other values
    vec4 deltaAB = step(abs(a-b), epsilon);
    return deltaAB.x * deltaAB.y * deltaAB.z * deltaAB.w;
}

vec4 scale2x(sampler2D tex, vec2 uv) {
    vec2 texSize = vec2(textureSize(tex, 0));
    vec2 o = 0.33 / texSize;

	// A B C
	// D E F
	// G H I
	vec4 B = texture(tex, uv + vec2(  0.0,  o.y));
	vec4 D = texture(tex, uv + vec2( -o.x,  0.0));
	vec4 E = texture(tex, uv + vec2(  0.0,  0.0));
	vec4 F = texture(tex, uv + vec2(  o.x,  0.0));
	vec4 H = texture(tex, uv + vec2(  0.0, -o.y));
	vec2 p = uv * texSize;

    //0 1
    //2 3
    vec4 E0, E1, E2, E3;
    float BdfH = 1.0-vec4Equal(B, H);
    float DdfF = 1.0-vec4Equal(D, F);
    float master = BdfH * DdfF;
    E0 = mix(E, D, vec4Equal(D, B) * master);
    E1 = mix(E, F, vec4Equal(B, F) * master);
    E2 = mix(E, D, vec4Equal(D, H) * master);
    E3 = mix(E, F, vec4Equal(H, F) * master);
	p = p - floor(p);
    return mix(
        mix(
            E2,
            E0,
        step(0.5, p.y)),
        mix(
            E3,
            E1,
        step(0.5, p.y)),
    step(0.5, p.x));
}
