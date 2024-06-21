// This is basically just Blender's Noise Texture shader node, ported from:
// <https://github.com/blender/blender/blob/594f47ecd2d5367ca936cf6fc6ec8168c2b360d0/intern/cycles/kernel/shaders/node_noise_texture.osl>
// That shader is written on OSL, reference: <https://raw.githubusercontent.com/imageworks/OpenShadingLanguage/master/src/doc/osl-languagespec.pdf>
// This one hardcodes a few values and removes some additional functionality so it would run a bit faster.

#include "./3d_noise.glsl"

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
  /* if (isinf(f)) */
  /*   return 0.0; */
  /* return f; */
}

// Originally was unsigned Perlin noise, in (0, 1)
// Simplex noise is close enough and faster
float safe_noise(vec3 p) {
  return (simplex_noise(p) + 1.)/2.;
  /* if (isinf(f)) */
  /*   return 0.5; */
  /* return f; */
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
