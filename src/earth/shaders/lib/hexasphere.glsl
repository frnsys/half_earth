// https://shaderfrog.com/app/view/2977?view=shader

/**
* Example Fragment Shader
* Sets the color and alpha of the pixel by setting gl_FragColor
*/

// Set the precision for data types used in this shader
precision highp float;
precision highp int;

// A uniform unique to this shader. You can modify it to the using the form
// below the shader preview. Any uniform you add is automatically given a form
const float hexMult = 1.;
const float coreSize = 0.;
const float width = 0.03;

const vec3 all0 = vec3(0.0);

const float SQRT3 = sqrt(3.0);
const float SQRT3_2 = 0.5 * SQRT3;

const vec2 P0 = vec2(-0.5, -0.5);
const vec2 P1 = vec2(0.5, -0.5);
const vec2 P2 = vec2(0.0, SQRT3_2 - 0.5);

float hex2(vec2 p, float width, float coreSize)
{
    p.x *= 0.57735 * 2.0;
    p.y += mod(floor(p.x), 2.0)*0.5;
    p = abs((mod(p, 1.0) - 0.5));
    float val = abs(max(p.x*1.5 + p.y, p.y*2.0) - 1.0);
    return smoothstep(coreSize, width, val);
}

vec3 getTriangleBarycentric(vec2 p, vec2 p0, vec2 p1, vec2 p2) {
    vec2 v0 = p2 - p0;
    vec2 v1 = p1 - p0;
    vec2 v2 = p - p0;

    float dot00 = dot(v0, v0);
    float dot01 = dot(v0, v1);
    float dot02 = dot(v0, v2);
    float dot11 = dot(v1, v1);
    float dot12 = dot(v1, v2);

    float invDenom = 1.0 / (dot00 * dot11 - dot01 * dot01);

    float s = (dot11 * dot02 - dot01 * dot12) * invDenom;
    float t = (dot00 * dot12 - dot01 * dot02) * invDenom;
    float q = 1.0 - s - t;
    return vec3(s, t, q);
}

vec3 getTriangleBarycentric(vec3 p, vec3 p0, vec3 p1, vec3 p2) {
    vec3 v0 = p2 - p0;
    vec3 v1 = p1 - p0;
    vec3 v2 = p - p0;

    float dot00 = dot(v0, v0);
    float dot01 = dot(v0, v1);
    float dot02 = dot(v0, v2);
    float dot11 = dot(v1, v1);
    float dot12 = dot(v1, v2);

    float invDenom = 1.0 / (dot00 * dot11 - dot01 * dot01);

    float s = (dot11 * dot02 - dot01 * dot12) * invDenom;
    float t = (dot00 * dot12 - dot01 * dot02) * invDenom;
    float q = 1.0 - s - t;
    return vec3(s, t, q);
}

// copy-pasted from inigo's : https://iquilezles.org/www/articles/distfunctions/distfunctions.htm
float dot2( in vec3 v ) { return dot(v,v); }
float DistancePointToTriangle( vec3 p, vec3 a, vec3 b, vec3 c )
{
    vec3 ba = b - a; vec3 pa = p - a;
    vec3 cb = c - b; vec3 pb = p - b;
    vec3 ac = a - c; vec3 pc = p - c;
    vec3 nor = cross( ba, ac );

    return sqrt(
    (sign(dot(cross(ba,nor),pa)) +
     sign(dot(cross(cb,nor),pb)) +
     sign(dot(cross(ac,nor),pc))<2.0)
     ?
     min( min(
        dot2(ba * clamp(dot(ba, pa) / dot2(ba), 0.0, 1.0) - pa),
        dot2(cb * clamp(dot(cb, pb) / dot2(cb), 0.0, 1.0) - pb) ),
        dot2(ac * clamp(dot(ac, pc) / dot2(ac), 0.0, 1.0) - pc) )
     :
     dot(nor,pa)*dot(nor,pa)/dot2(nor) );
}

float triangle2(vec3 p, vec3 p0, vec3 p1, vec3 p2)
{
    float s1 = sign( dot(cross(p0, p1), p2) );
    float s2 = sign( dot(cross(p , p1), p2) );
    float s3 = sign( dot(cross(p0, p ), p2) );
    float s4 = sign( dot(cross(p0, p1), p ) );
    return float(all(equal(vec3(s2, s3, s4), vec3(s1))));
}

//cutoff like here: https://stackoverflow.com/questions/46777626/mathematically-producing-sphere-shaped-hexagonal-grid
float triangledHex(vec2 uvOrig, float uvMul, float width, float coreSize)
{
    vec2 myuv = uvOrig;

    //must be even number to cut it right
    uvMul *= 2.0;

    myuv -= vec2(0.5, 0.0);

    //TODO figure out what 3.22 is!!!
    myuv -= vec2(0.0, 1.0 - SQRT3_2) / uvMul * 3.22;
    myuv *= uvMul;

    vec2 uvTriangle = uvOrig - vec2(0.5);

    return hex2( myuv.yx, width, coreSize);
}


mat3 icoFace1 = mat3(  -0.52573111211913, 0.0, 0.85065080835204,
                        0.0, 0.85065080835204, 0.52573111211913,
                        0.52573111211913, 0.0, 0.85065080835204);

mat3 icoFace2 = mat3(  -0.52573111211913, 0.0, 0.85065080835204,
                        -0.85065080835204, 0.52573111211913, 0.0,
                        0.0, 0.85065080835204, 0.52573111211913);

mat3 icoFace3 = mat3(  -0.85065080835204, 0.52573111211913, 0.0,
                        0.0, 0.85065080835204, -0.52573111211913,
                        0.0, 0.85065080835204, 0.52573111211913);

mat3 icoFace4 = mat3(  0.0, 0.85065080835204, 0.52573111211913,
                        0.0, 0.85065080835204, -0.52573111211913,
                        0.85065080835204, 0.52573111211913, 0.0);

mat3 icoFace5 = mat3(  0.0, 0.85065080835204, 0.52573111211913,
                        0.85065080835204, 0.52573111211913, 0.0,
                        0.52573111211913, 0.0, 0.85065080835204);

mat3 icoFace6 = mat3(  0.85065080835204, 0.52573111211913, 0.0,
                        0.85065080835204, -0.52573111211913, 0.0,
                        0.52573111211913, 0.0, 0.85065080835204);

mat3 icoFace7 = mat3(  0.85065080835204, 0.52573111211913, 0.0,
                        0.52573111211913, 0.0, -0.85065080835204,
                        0.85065080835204, -0.52573111211913, 0.0);

mat3 icoFace8 = mat3(  0.0, 0.85065080835204, -0.52573111211913,
                        0.52573111211913, 0.0, -0.85065080835204,
                        0.85065080835204, 0.52573111211913, 0.0);

mat3 icoFace9 = mat3(  0.0, 0.85065080835204, -0.52573111211913,
                        -0.52573111211913, 0.0, -0.85065080835204,
                        0.52573111211913, 0.0, -0.85065080835204);

mat3 icoFace10 = mat3( -0.52573111211913, 0.0, -0.85065080835204,
                        0.0, -0.85065080835204, -0.52573111211913,
                        0.52573111211913, 0.0, -0.85065080835204);

mat3 icoFace11 = mat3(  0.0, -0.85065080835204, -0.52573111211913,
                        0.85065080835204, -0.52573111211913, 0.0,
                        0.52573111211913, 0.0, -0.85065080835204);


mat3 icoFace12 = mat3( 0.0, -0.85065080835204, -0.52573111211913,
                        0.0, -0.85065080835204, 0.52573111211913,
                        0.85065080835204, -0.52573111211913, 0.0);

mat3 icoFace13 = mat3( 0.0, -0.85065080835204, -0.52573111211913,
                        -0.85065080835204, -0.52573111211913, 0.0,
                        0.0, -0.85065080835204, 0.52573111211913);

mat3 icoFace14 = mat3( -0.85065080835204, -0.52573111211913, 0.0,
                        -0.52573111211913, 0.0, 0.85065080835204,
                        0.0, -0.85065080835204, 0.52573111211913);

mat3 icoFace15 = mat3( -0.52573111211913, 0.0, 0.85065080835204,
                        0.52573111211913, 0.0, 0.85065080835204,
                        0.0, -0.85065080835204, 0.52573111211913);

mat3 icoFace16 = mat3( 0.0, -0.85065080835204, 0.52573111211913,
                        0.52573111211913, 0.0, 0.85065080835204,
                        0.85065080835204, -0.52573111211913, 0.0);

mat3 icoFace17 = mat3( -0.85065080835204, 0.52573111211913, 0.0,
                        -0.52573111211913, 0.0, 0.85065080835204,
                        -0.85065080835204, -0.52573111211913, 0.0);

mat3 icoFace18 = mat3( -0.85065080835204, 0.52573111211913, 0.0,
                        -0.85065080835204, -0.52573111211913, 0.0,
                        -0.52573111211913, 0.0, -0.85065080835204);

mat3 icoFace19 = mat3( -0.85065080835204, 0.52573111211913, 0.0,
                        -0.52573111211913, 0.0, -0.85065080835204,
                        0.0, 0.85065080835204, -0.52573111211913);

mat3 icoFace20 = mat3( 0.0, -0.85065080835204, -0.52573111211913,
                        -0.52573111211913, 0.0, -0.85065080835204,
                        -0.85065080835204, -0.52573111211913, 0.0);

float faceValue(mat3 icoFace, vec3 pos)
{
    #if 1
        float triangleToPointDist = DistancePointToTriangle(pos, icoFace[0], icoFace[1], icoFace[2]);
        pos = pos * (1.0 - triangleToPointDist); //projection from sphere to planar icosahedron's triangle
    #else
        vec3 normToTriangle = normalize(cross(icoFace[2] - icoFace[0], icoFace[1] - icoFace[0]));
        float triangleToPointDist = length( dot((pos - icoFace[0]), normToTriangle) * normToTriangle );
        pos = pos * (1.0 - triangleToPointDist);
    #endif

    vec3 baryP = getTriangleBarycentric(pos, icoFace[0], icoFace[1], icoFace[2]);

    vec2 planarPoint = (baryP.x * vec2(0.0, 0.0) + baryP.y * vec2(1.0, 0.0) + baryP.z * vec2(0.5, SQRT3_2));

    return triangledHex(planarPoint, floor(hexMult + 0.5), width, coreSize);
}

mat3 getIcoFace(int i)
{
    if (i == 1)
        return icoFace1;
    else if (i == 2)
        return icoFace2;
    else if (i == 3)
        return icoFace3;
    else if (i == 4)
        return icoFace4;
    else if (i == 5)
        return icoFace5;
    else if (i == 6)
        return icoFace6;
    else if (i == 7)
        return icoFace7;
    else if (i == 8)
        return icoFace8;
    else if (i == 9)
        return icoFace9;
    else if (i == 10)
        return icoFace10;
    else if (i == 11)
        return icoFace11;
    else if (i == 12)
        return icoFace12;
    else if (i == 13)
        return icoFace13;
    else if (i == 14)
        return icoFace14;
    else if (i == 15)
        return icoFace15;
    else if (i == 16)
        return icoFace16;
    else if (i == 17)
        return icoFace17;
    else if (i == 18)
        return icoFace18;
    else if (i == 19)
        return icoFace19;
    else //20
        return icoFace20;
}

vec3 hexasphere() {
    mat3 icoFace;
    float tr;
    float hexVal;

    for (int i = 0; i <= 20; ++i)
    {
        icoFace = getIcoFace(i);
        tr = triangle2(vertexPosition, icoFace[0], icoFace[1], icoFace[2]);
        if (tr == 1.0) {
            hexVal = faceValue(icoFace, vertexPosition);
            break;
        }
    }
    return vec3(tr * hexVal);
}

