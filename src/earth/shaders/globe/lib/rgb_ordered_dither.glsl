// https://www.shadertoy.com/view/NtS3zd

const float GradientColorQuantity = 3.;
const float noisePercent = 0.2;

vec3 BeforeColorPalette[] = vec3[](
    vec3(0.102, 0.11, 0.173),
    vec3(0.36,0.15,0.36),
    vec3(0.69,0.24,0.33),
    vec3(0.94,0.49,0.34),

    vec3(1.0,0.8,0.46),
    vec3(0.65,0.94,0.44),
    vec3(0.22,0.72,0.39),
    vec3(0.15,0.44,0.47),

    vec3(0.16,0.21,0.44),
    vec3(0.23,0.36,0.79),
    vec3(0.25,0.65,0.96),
    vec3(0.45,0.94,0.97),

    vec3(0.96,0.96,0.96),
    vec3(0.58,0.69,0.76),
    vec3(0.34,0.42,0.53),
    vec3(0.2,0.24,0.34)
);

vec3 AfterColorPalette[] = vec3[](
    vec3(0.102, 0.11, 0.173),
    vec3(0.36,0.15,0.36),
    vec3(0.69,0.24,0.33),
    vec3(0.94,0.49,0.34),

    vec3(1.0,0.8,0.46),
    vec3(0.65,0.94,0.44),
    vec3(0.22,0.72,0.39),
    vec3(0.15,0.44,0.47),

    vec3(0.16,0.21,0.44),
    vec3(0.23,0.36,0.79),
    vec3(0.25,0.65,0.96),
    vec3(0.45,0.94,0.97),

    vec3(0.96,0.96,0.96),
    vec3(0.58,0.69,0.76),
    vec3(0.34,0.42,0.53),
    vec3(0.2,0.24,0.34)

);

float GetNoise(vec2 uv){
    /* return fract(sin(uv.x+1000.0*uv.y+iTime*20.0)*10000.0)*noisePercent; */
    return fract(sin(uv.x+1000.0*uv.y+20.0)*10000.0)*noisePercent;
}

float Get(float pixelPosition){
    return(mod(pixelPosition,GradientColorQuantity)/GradientColorQuantity);
}

float GetR(float pixelPosition)
{
    return Get(pixelPosition + GradientColorQuantity*1./3.);
}

float GetG(float pixelPosition)
{
    return Get(pixelPosition + GradientColorQuantity*2./3.);
}

float GetB(float pixelPosition)
{
    return Get(pixelPosition + GradientColorQuantity*3./3.);
}

vec3 GetNearestColorPalette(vec3 currentColor,vec3 colorPalette[16]) {
    float nearestDistance = 10000.0;
    vec3 nearestColor;
    for(int i ; i<colorPalette.length() ; i++){
       vec3 newColor = colorPalette[i];
       float newDistance = distance(currentColor,newColor);
       if(newDistance<=nearestDistance){
           nearestColor = newColor;
           nearestDistance = newDistance;
       }
    }
    return nearestColor;
}

vec3 rgbOrderedDither(vec3 color) {
    vec3 paletteColor = GetNearestColorPalette(color, BeforeColorPalette);
    //calculate a gradient x and y
    vec3 stepEdgeX = vec3(GetR(gl_FragCoord.x),GetG(gl_FragCoord.x),GetB(gl_FragCoord.x));
    vec3 stepEdgeY = vec3(GetR(gl_FragCoord.y),GetG(gl_FragCoord.y),GetB(gl_FragCoord.y));
    vec3 valueEdge = ((stepEdgeX + stepEdgeY) + (stepEdgeX * stepEdgeY))/2.; // simple average of (subtractive and aditive)

    // dithering on the gradient
    vec3 newColor = step(valueEdge+GetNoise(vertexUV), paletteColor);
    return GetNearestColorPalette(newColor,AfterColorPalette);
}
