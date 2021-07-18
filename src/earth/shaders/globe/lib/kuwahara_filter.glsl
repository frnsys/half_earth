// <https://www.shadertoy.com/view/MlyfWd>
// TODO
const int half_width = 5;

// Calculate color distance
float CalcDistance(in vec3 c0, in vec3 c1) {
    vec3 sub = c0 - c1;
    return dot(sub, sub);
}

// Symmetric Nearest Neighbor
vec3 CalcSNN(in vec2 fragCoord) {
	vec2 src_size = iResolution.xy;
    vec2 inv_src_size = 1.0f / src_size;
    vec2 uv = fragCoord * inv_src_size;

    vec3 c0 = texture(iChannel0, uv).rgb;

    vec4 sum = vec4(0.0f, 0.0f, 0.0f, 0.0f);

    for (int i = 0; i <= half_width; ++i) {
        vec3 c1 = texture(iChannel0, uv + vec2(+i, 0) * inv_src_size).rgb;
        vec3 c2 = texture(iChannel0, uv + vec2(-i, 0) * inv_src_size).rgb;

        float d1 = CalcDistance(c1, c0);
        float d2 = CalcDistance(c2, c0);
        if (d1 < d2) {
            sum.rgb += c1;
        } else {
            sum.rgb += c2;
        }
        sum.a += 1.0f;
    }
 	for (int j = 1; j <= half_width; ++j) {
    	for (int i = -half_width; i <= half_width; ++i) {
            vec3 c1 = texture(iChannel0, uv + vec2(+i, +j) * inv_src_size).rgb;
            vec3 c2 = texture(iChannel0, uv + vec2(-i, -j) * inv_src_size).rgb;

            float d1 = CalcDistance(c1, c0);
            float d2 = CalcDistance(c2, c0);
            if (d1 < d2) {
            	sum.rgb += c1;
            } else {
                sum.rgb += c2;
            }
            sum.a += 1.0f;
		}
    }
    return sum.rgb / sum.a;
}

// Kuwahara
vec3 kuwahara_filter(in vec2 fragCoord) {
    vec2 src_size = iResolution.xy;
    vec2 inv_src_size = 1.0f / src_size;
    vec2 uv = fragCoord * inv_src_size;

    float n = float((half_width + 1) * (half_width + 1));
    float inv_n = 1.0f / n;

    vec3 col = vec3(0, 0, 0);

    float sigma2 = 0.0f;
    float min_sigma = 100.0f;

    vec3 m = vec3(0, 0, 0);
    vec3 s = vec3(0, 0, 0);


    for (int j = -half_width; j <= 0; ++j) {
        for (int i = -half_width; i <= 0; ++i) {
            vec3 c = texture(iChannel0, uv + vec2(i, j) * inv_src_size).rgb;
            m += c;
            s += c * c;
        }
    }

    m *= inv_n;
    s = abs(s * inv_n - m * m);

    sigma2 = s.x + s.y + s.z;
    if (sigma2 < min_sigma) {
        min_sigma = sigma2;
        col = m;
    }

    m = vec3(0, 0, 0);
    s = vec3(0, 0, 0);

    for (int j = -half_width; j <= 0; ++j) {
        for (int i = 0; i <= half_width; ++i) {
            vec3 c = texture(iChannel0, uv + vec2(i, j) * inv_src_size).rgb;
            m += c;
            s += c * c;
        }
    }

    m *= inv_n;
    s = abs(s * inv_n - m * m);

    sigma2 = s.x + s.y + s.z;
    if (sigma2 < min_sigma) {
        min_sigma = sigma2;
        col = m;
    }

    m = vec3(0, 0, 0);
    s = vec3(0, 0, 0);

    for (int j = 0; j <= half_width; ++j) {
        for (int i = 0; i <= half_width; ++i) {
            vec3 c = texture(iChannel0, uv + vec2(i, j) * inv_src_size).rgb;
            m += c;
            s += c * c;
        }
    }

    m *= inv_n;
    s = abs(s * inv_n - m * m);

    sigma2 = s.x + s.y + s.z;
    if (sigma2 < min_sigma) {
        min_sigma = sigma2;
        col = m;
    }

    m = vec3(0, 0, 0);
    s = vec3(0, 0, 0);

    for (int j = 0; j <= half_width; ++j) {
        for (int i = -half_width; i <= 0; ++i) {
            vec3 c = texture(iChannel0, uv + vec2(i, j) * inv_src_size).rgb;
            m += c;
            s += c * c;
        }
    }

    m *= inv_n;
    s = abs(s * inv_n - m * m);

    sigma2 = s.x + s.y + s.z;
    if (sigma2 < min_sigma) {
        min_sigma = sigma2;
        col = m;
    }

    return col;
}
