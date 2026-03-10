// helper functions
var<private> pi:f32 = 3.14159265359;
var<private> e:f32 = 2.71828182845;

fn c_add(a:vec2f, s:f32) -> vec2f{
    return vec2f(a.x+s, a.y);
}
fn c_mul(a:vec2f, b:vec2f) ->vec2f{
    return vec2f(a.x*b.x-a.y*b.y, a.x*b.y + a.y*b.x);
}
fn c_div(a:vec2f, b:vec2f) ->vec2f{
    let d:f32 = dot(b,b);
    return vec2f(dot(a,b)/d, (a.y*b.x-a.x*b.y)/d);
}
fn c_sqrt(z:vec2f) -> vec2f{
    let m:f32 = length(z);
    let s = sqrt(0.5*vec2f(m+z.x, m-z.x));
    return s*vec2f(1.0, sign(z.y));
}
fn c_conj(z:vec2f) -> vec2f{
    return vec2f(z.x, -z.y);
}
fn c_pow (z:vec2f, n:f32) -> vec2f{
    let r:f32 = length(z);
    let a:f32 = atan2(z.y, z.x);
    return pow(r, n) * vec2f(cos(a*n), sin(a*n)); 
}
fn c_inv(z:vec2f) -> vec2f{
    return vec2f(z.x/dot(z,z), -z.y/dot(z,z));
}
fn c_arg(z: vec2f) -> f32{
    var f:f32 = atan2(z.y, z.x);
    if(f<0.0){
        f = f + 6.2831;
    }
    return f/6.2831;
}
fn c_log(z:vec2f) -> vec2f{
    return vec2f(log(sqrt(dot(z,z))), atan2(z.y, z.x));
}
fn c_sin(z:vec2f) ->vec2f{
    let a = pow(e, z.y);
    let b = pow(e, -z.y);
    return vec2f(sin(z.x)*(a+b)*0.5, cos(z.x)*(a-b)*0.5);
}
fn c_cos(z:vec2f) ->vec2f{
    let a = pow(e, z.y);
    let b = pow(e, -z.y);
    return vec2f(cos(z.x)*(a+b)*0.5, -sin(z.x)*(a-b)*0.5);
}
fn c_tan(z:vec2f) ->vec2f{
    let a = pow(e, z.y);
    let b = pow(e, -z.y);
    let cx = cos(z.x);
    let ab = (a - b)*0.5;
    return vec2f(sin(z.x)*cx, ab*(a+b)*0.5)/(cx*cx+ab*ab);
}
fn c_exp2(z:vec2f) ->vec2f{
    return vec2f(z.x*z.x - z.y*z.y, 2.*z.x*z.y);
}

fn c_exp(z:vec2f) ->vec2f{
    return vec2f(exp(z.x)*cos(z.y), exp(z.x)*sin(z.y));
}

fn c_asinh(z:vec2f) -> vec2f{
    let a = z + c_sqrt(c_mul(z,z) + vec2f(1.0,0.0));
    return c_log(a);
}

// hsv to rgb color conversion
fn hsv_to_rgb(z:vec2f) -> vec3f{
    let len = length(z);
    let h = c_arg(z);
    var fx = 2.0*(fract(z.x) - 0.5);
    var fy = 2.0*(fract(z.y) - 0.5);
    fx = fx*fx;
    fy = fy*fy;
    var g = 1.0 -(1.0 - fx)*(1.0 - fy);
    g = pow(abs(g), 10.0);
    var c = 2.0*(fract(log2(len)) - 0.5);
    c = 0.7*pow(abs(c), 10.0);  
    var v = 1.0 - 0.5*g;
    let f = abs((h*6.0 + vec3f(0.0,4.0,2.0))%6.0 - 3.0) - 1.0;
    var rgb = clamp(f, vec3f(0.0,0.0,0.0), vec3f(1.0,1.0,1.0));
    rgb = rgb*rgb*(3.0 - 2.0*rgb);
    rgb = (1.0-c)*v*mix(vec3f(1.0, 1.0, 1.0), rgb, 1.0);  
    return rgb + c*vec3f(1.0,1.0,1.0);
}

fn hsv_to_rgb1(z:vec2f) -> vec3f{
    let len = length(z);
    let h = c_arg(z);
    let s = abs(sin(2.0*pi*len));
    let b = pow(abs(sin(2.0*pi*z.y)*sin(2.0*pi*z.x)), 0.25);
    let v = 0.5*((1.0-s) + b + sqrt((1.0-s-b)*(1.0-s-b) + 0.01));
    let f:vec3f = abs((h*6.0 + vec3f(0.0,4.0,2.0))%6.0 - 3.0) - 1.0;
    var rgb:vec3f = clamp(f, vec3f(0.0,0.0,0.0), vec3f(1.0,1.0,1.0));
    rgb = rgb*rgb*(3.0 - 2.0*rgb);
    return v*mix(vec3f(1.0, 1.0, 1.0), rgb, s);  
}

// colormap
fn colormap(colormap_id:i32) -> array<vec3f, 11>{
    var colors:array<vec3f, 11>;
    switch (colormap_id) {
        case 1, default: { // jet
            colors = array<vec3f, 11>(
                vec3f(0.0,0.0,0.51),
                vec3f(0.0,0.24,0.67),
                vec3f(0.01,0.49,0.78),
                vec3f(0.01,0.75,0.89),
                vec3f(0.02,1.0,1.0),
                vec3f(0.51,1.0,0.5),
                vec3f(1.0,1.0,0.0),
                vec3f(0.99,0.67,0.0),
                vec3f(0.99,0.33,0.0),
                vec3f(0.98,0.0,0.0),
                vec3f(0.5,0.0,0.0));
        }
        case 2 : { // hsv
            colors = array<vec3f, 11>(
                vec3f(1.0,0.0,0.0),
                vec3f(1.0,0.5,0.0),
                vec3f(0.97,1.0,0.01),
                vec3f(0.0,0.99,0.04),
                vec3f(0.0,0.98,0.52),
                vec3f(0.0,0.98,1.0),
                vec3f(0.01,0.49,1.0),
                vec3f(0.03,0.0,0.99),
                vec3f(1.0,0.0,0.96),
                vec3f(1.0,0.0,0.49),
                vec3f(1.0,0.0,0.02));
            break;
        }
        case 3: { // hot
            colors = array<vec3f, 11>(
                vec3f(0.0,0.0,0.0),
                vec3f(0.3,0.0,0.0),
                vec3f(0.6,0.0,0.0),
                vec3f(0.9,0.0,0.0),
                vec3f(0.93,0.0,0.0),
                vec3f(0.97,0.55,0.0),
                vec3f(1.0,0.82,0.0),
                vec3f(1.0,0.87,0.25),
                vec3f(1.0,0.91,0.5),
                vec3f(1.0,0.96,0.75),
                vec3f(1.0,1.0,1.0));
            break;
        }
        case 4: { // cool
            colors = array<vec3f, 11>(
                vec3f(0.49,0.0,0.7),
                vec3f(0.45,0.0,0.85),
                vec3f(0.42,0.15,0.89),
                vec3f(0.38,0.29,0.93),
                vec3f(0.27,0.57,0.910),
                vec3f(0.0,0.8,0.77),
                vec3f(0.0,0.97,0.57),
                vec3f(0.0,0.98,0.46),
                vec3f(0.0,1.0,0.35),
                vec3f(0.16,1.0,0.03),
                vec3f(0.58,1.0,0.0));
            break;
        }
        case 5: { // spring
            colors = array<vec3f, 11>(
                vec3f(1.0,0.0,1.0),
                vec3f(1.0,0.1, 0.9),
                vec3f(1.0,0.2,0.8),
                vec3f(1.0,0.3,0.7),
                vec3f(1.0,0.4,0.6),
                vec3f(1.0,0.5,0.5),
                vec3f(1.0,0.6,0.4),
                vec3f(1.0,0.7,0.3),
                vec3f(1.0,0.8,0.2),
                vec3f(1.0,0.9,0.1),
                vec3f(1.0,1.0,0.0));
            break;
        }
        case 6: { // summer
            colors = array<vec3f, 11>(
                vec3f(0.0,0.5,0.4),
                vec3f(0.1,0.55,0.4),
                vec3f(0.2,0.6,0.4),
                vec3f(0.3,0.65,0.4),
                vec3f(0.4,0.7,0.4),
                vec3f(0.5,0.75,0.4),
                vec3f(0.6,0.8,0.4),
                vec3f(0.7,0.85,0.4),
                vec3f(0.8,0.9,0.4),
                vec3f(0.9,0.95,0.4),
                vec3f(1.0,1.0,0.4));
            break;
        }
        case 7: { // autumn
            colors = array<vec3f, 11>(
                vec3f(1.0,0.0,0.0),
                vec3f(1.0,0.1,0.0),
                vec3f(1.0,0.2,0.0),
                vec3f(1.0,0.3,0.0),
                vec3f(1.0,0.4,0.0),
                vec3f(1.0,0.5,0.0),
                vec3f(1.0,0.6,0.0),
                vec3f(1.0,0.7,0.0),
                vec3f(1.0,0.8,0.0),
                vec3f(1.0,0.9,0.0),
                vec3f(1.0,1.0,0.0));
            break;
        }
        case 8: { // winter
            colors = array<vec3f, 11>(
                vec3f(0.0,0.0,1.0),
                vec3f(0.0,0.1,0.95),
                vec3f(0.0,0.2,0.9),
                vec3f(0.0,0.3,0.85),
                vec3f(0.0,0.4,0.8),
                vec3f(0.0,0.5,0.75),
                vec3f(0.0,0.6,0.7),
                vec3f(0.0,0.7,0.65),
                vec3f(0.0,0.8,0.6),
                vec3f(0.0,0.9,0.55),
                vec3f(0.0,1.0,0.5));
            break;
        }
        case 9: { // bone
            colors = array<vec3f, 11>(
                vec3f(0.0,0.0,0.0),
                vec3f(0.08,0.08,0.11),
                vec3f(0.16,0.16,0.23),
                vec3f(0.25,0.25,0.34),
                vec3f(0.33,0.33,0.45),
                vec3f(0.41,0.44,0.54),
                vec3f(0.5,0.56,0.62),
                vec3f(0.58,0.67,0.7),
                vec3f(0.66,0.78,0.78),
                vec3f(0.83,0.89,0.89),
                vec3f(1.0,1.0,1.0));
            break;
        }
        case 10: { // cooper
            colors = array<vec3f, 11>(
                vec3f(0.0,0.0,0.0),
                vec3f(0.13,0.08,0.05),
                vec3f(0.25,0.16,0.1),
                vec3f(0.38,0.24,0.15),
                vec3f(0.5,0.31,0.2),
                vec3f(0.62,0.39,0.25),
                vec3f(0.75,0.47,0.3),
                vec3f(0.87,0.55,0.35),
                vec3f(1.0,0.63,0.4),
                vec3f(1.0,0.71,0.45),
                vec3f(1.0,0.78,0.5));
            break;
        }
        case 11: { // greys
            colors = array<vec3f, 11>(
                vec3f(0.0,0.0,0.0),
                vec3f(0.1,0.1,0.1),
                vec3f(0.2,0.2,0.2),
                vec3f(0.3,0.3,0.3),
                vec3f(0.4,0.4,0.4),
                vec3f(0.5,0.5,0.5),
                vec3f(0.6,0.6,0.6),
                vec3f(0.7,0.7,0.7),
                vec3f(0.8,0.8,0.8),
                vec3f(0.9,0.9,0.9),
                vec3f(1.0,1.0,1.0));
            break;
        }
    }
    return colors;
}

fn colormap_to_rgb(z:vec2f, colormap_id:i32) -> vec3f{
    var c = colormap(colormap_id);
    let len = length(z);
    var h = atan2(z.y, z.x);
    if(h < 0.0) { h = h + 2.0*pi; }
    if(h >= 2.0*pi) { h = h - 2.0*pi; }
    var s:f32 = 0.0;
    var v = vec3f(0.0, 0.0, 0.0);

    for(var i:i32=0; i<11; i=i+1){
        if(h>=0.2*pi*f32(i) && h < 0.2*pi*(f32(i)+1.0)){
            s = (h - f32(i)*0.2*pi)/(0.2*pi);
            v = s*c[i+1] + (1.0-s)*c[i];
        }
    }
    let b = fract(log2(len));
    return vec3f(v[0]*b, v[1]*b, v[2]*b);
}
