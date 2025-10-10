layout (location = 0) out vec4 outColor;

uniform float time;

in vec3 pos;

// ==========================================
// https://github.com/hughsk/glsl-noise
vec3 mod289(vec3 x) {
  return x - floor(x * (1.0 / 289.0)) * 289.0;
}
vec4 mod289(vec4 x) {
  return x - floor(x * (1.0 / 289.0)) * 289.0;
}
vec4 permute(vec4 x) {
     return mod289(((x*34.0)+1.0)*x);
}
vec4 taylorInvSqrt(vec4 r) {
  return 1.79284291400159 - 0.85373472095314 * r;
}
// ==========================================


// ==========================================
// Description : Array and textureless GLSL 2D/3D/4D simplex
//               noise functions.
//      Author : Ian McEwan, Ashima Arts.
//  Maintainer : ijm
//     Lastmod : 20110822 (ijm)
//     License : Copyright (C) 2011 Ashima Arts. All rights reserved.
//               Distributed under the MIT License. See LICENSE file.
//               https://github.com/ashima/webgl-noise
float simplex_noise(vec3 v)
  {
  const vec2  C = vec2(1.0/6.0, 1.0/3.0) ;
  const vec4  D = vec4(0.0, 0.5, 1.0, 2.0);

// First corner
  vec3 i  = floor(v + dot(v, C.yyy) );
  vec3 x0 =   v - i + dot(i, C.xxx) ;

// Other corners
  vec3 g = step(x0.yzx, x0.xyz);
  vec3 l = 1.0 - g;
  vec3 i1 = min( g.xyz, l.zxy );
  vec3 i2 = max( g.xyz, l.zxy );

  //   x0 = x0 - 0.0 + 0.0 * C.xxx;
  //   x1 = x0 - i1  + 1.0 * C.xxx;
  //   x2 = x0 - i2  + 2.0 * C.xxx;
  //   x3 = x0 - 1.0 + 3.0 * C.xxx;
  vec3 x1 = x0 - i1 + C.xxx;
  vec3 x2 = x0 - i2 + C.yyy; // 2.0*C.x = 1/3 = C.y
  vec3 x3 = x0 - D.yyy;      // -1.0+3.0*C.x = -0.5 = -D.y

// Permutations
  i = mod289(i);
  vec4 p = permute( permute( permute(
             i.z + vec4(0.0, i1.z, i2.z, 1.0 ))
           + i.y + vec4(0.0, i1.y, i2.y, 1.0 ))
           + i.x + vec4(0.0, i1.x, i2.x, 1.0 ));

// Gradients: 7x7 points over a square, mapped onto an octahedron.
// The ring size 17*17 = 289 is close to a multiple of 49 (49*6 = 294)
  float n_ = 0.142857142857; // 1.0/7.0
  vec3  ns = n_ * D.wyz - D.xzx;

  vec4 j = p - 49.0 * floor(p * ns.z * ns.z);  //  mod(p,7*7)

  vec4 x_ = floor(j * ns.z);
  vec4 y_ = floor(j - 7.0 * x_ );    // mod(j,N)

  vec4 x = x_ *ns.x + ns.yyyy;
  vec4 y = y_ *ns.x + ns.yyyy;
  vec4 h = 1.0 - abs(x) - abs(y);

  vec4 b0 = vec4( x.xy, y.xy );
  vec4 b1 = vec4( x.zw, y.zw );

  //vec4 s0 = vec4(lessThan(b0,0.0))*2.0 - 1.0;
  //vec4 s1 = vec4(lessThan(b1,0.0))*2.0 - 1.0;
  vec4 s0 = floor(b0)*2.0 + 1.0;
  vec4 s1 = floor(b1)*2.0 + 1.0;
  vec4 sh = -step(h, vec4(0.0));

  vec4 a0 = b0.xzyw + s0.xzyw*sh.xxyy ;
  vec4 a1 = b1.xzyw + s1.xzyw*sh.zzww ;

  vec3 p0 = vec3(a0.xy,h.x);
  vec3 p1 = vec3(a0.zw,h.y);
  vec3 p2 = vec3(a1.xy,h.z);
  vec3 p3 = vec3(a1.zw,h.w);

//Normalise gradients
  vec4 norm = taylorInvSqrt(vec4(dot(p0,p0), dot(p1,p1), dot(p2, p2), dot(p3,p3)));
  p0 *= norm.x;
  p1 *= norm.y;
  p2 *= norm.z;
  p3 *= norm.w;

// Mix final noise value
  vec4 m = max(0.6 - vec4(dot(x0,x0), dot(x1,x1), dot(x2,x2), dot(x3,x3)), 0.0);
  m = m * m;
  return 42.0 * dot( m*m, vec4( dot(p0,x0), dot(p1,x1),
                                dot(p2,x2), dot(p3,x3) ) );
  }
// ==========================================

// Shader runs faster the lower this is,
// but the clouds are blobbier
const int details = 3;
const float roughness = 0.625;
const float distortion = 1.050;

// Noise
// From: <https://gist.github.com/patriciogonzalezvivo/670c22f3966e662d2f83>
float hash(float x, float y) { return fract(1e4 * sin(17.0 * x + y * 0.1) * (0.1 + abs(sin(y * 13.0 + x)))); }

// Originally was Perlin noise in [-1, 1]
// Simplex noise is close enough and faster
float safe_snoise(vec3 p) {
  return simplex_noise(p);
}

// Originally was unsigned Perlin noise, in (0, 1)
// Simplex noise is close enough and faster
float safe_noise(vec3 p) {
  return (simplex_noise(p) + 1.)/2.;
}

float fractal_noise(vec3 p) {
  float fscale = 1.0;
  float amp = 1.0;
  float maxamp = 0.0;
  float sum = 0.0;
  for (int i = 0; i <= details; i++) {
    float t = safe_noise(fscale * p);
    sum += t * amp;
    maxamp += amp;
    amp *= roughness;
    fscale *= 2.0;
  }
  return sum / maxamp;
}


vec3 random_vec3_offset(float seed) {
  return vec3(100.0 + hash(seed, 0.0) * 100.0,
                 100.0 + hash(seed, 1.0) * 100.0,
                 100.0 + hash(seed, 2.0) * 100.0);
}


float noise_texture(vec3 co) {
  vec3 p = co;
  p += vec3(safe_snoise(p + random_vec3_offset(0.0)) * distortion,
            safe_snoise(p + random_vec3_offset(1.0)) * distortion,
            safe_snoise(p + random_vec3_offset(2.0)) * distortion);
  return fractal_noise(p);
}

float clouds(vec3 p, float scale) {
    return noise_texture(p * scale);
}

void main() {
    // Apply cloud layer
    float scale = 0.5 + sin(sqrt(time/2000.))/10.;
    float n = clouds(pos + time/100000., scale);
    outColor = n < 0.58 ? vec4(0.0) : vec4(n+0.35);
}
