var<private> backgr:bool = false;
var<private> max_iter:f32;
var<private> i_time:f32;

fn rotate(p:vec3f, thetaX:f32, thetaY:f32) -> vec3f{
    let cy = cos(thetaY);
    let sy = sin(thetaY);
    let r = vec3f(p.x, sy*p.z + cy*p.y, cy*p.z - sy*p.y);
    let cx = cos(thetaX);
    let sx = sin(thetaX);
    return -vec3f(cx*r.x - sx*r.z, r.y, sx*r.x + cx*r.z);
}

fn hsv2rgb(c:vec3f) ->vec3f{
    let f = abs((c.x*6.0 + vec3f(0.0,4.0,2.0))%6.0 - 3.0) - 1.0;
    let rgb = clamp(f, vec3f(0.0,0.0,0.0), vec3f(1.0,1.0,1.0));
    return vec3f(c.z*mix(vec3f(1.0, 1.0, 1.0), rgb, c.y));
}

fn dist_sphere(p:vec3f, r:f32) -> f32{
    return f32(length(p) - r);
}

fn dist_estimate(p:vec3f) ->f32{
    let bailout = 2.0;
    let d_sphere = -dist_sphere(p, 13.0);
    var v = p;
    var r = 0.0;
    var dr = 1.0;
    let power = abs(cos(i_time*0.1))*10.0 + 3.0;
    for(var n:f32 = 0.0; n <= max_iter; n=n+1.0){
        r = length(v);
        if(r > bailout) { break;}
        var theta = acos(v.z/r);
        var phi = atan2(v.y, v.x);
        dr = pow(r, power - 1.0) * power*dr + 1.0;

        let vr = pow(r, power);
        theta = theta*power;
        phi = phi*power;
        v = vr * vec3f(sin(theta)*cos(phi), sin(phi)*sin(theta), cos(theta));
        v = v + p;
    }
    let d_frac = 0.5*log(r)*r/dr;
    if(d_sphere < d_frac) {
        backgr = true;
    } else {
        backgr = false;
    }
    return min(d_frac, d_sphere);
}

fn get_normal(p:vec3f, d:f32) -> vec3f{
    let eps = vec3f(0.001, 0.0, 0.0);
    return normalize(vec3f(dist_estimate(p+eps.xyy), dist_estimate(p+eps.yxy), dist_estimate(p+eps.yyx)) - d);
}

fn soft_shadow(ro:vec3f, rd:vec3f, k:f32) -> f32{
    var res = 1.0;
    var t = 0.0;
    for(var i:i32 = 0; i<64; i=i+1){
        let d = dist_estimate(ro+rd*t);
        res = min(res, k*d/t);
        if(res<0.001) {break;}
        t = t + clamp(d, 0.01, 0.2);
    }
    return clamp(res, 0.0, 1.0);
}


// vertex shader
@vertex
fn vs_main(@builtin(vertex_index) in_vertex_index: u32) -> @builtin(position) vec4f {
    var pos = array<vec2f,4>(
        vec2f(-1.0, -1.0),
        vec2f( 1.0, -1.0),
        vec2f(-1.0,  1.0),
        vec2f( 1.0,  1.0),
    );
    return vec4f(pos[in_vertex_index], 0.0, 1.0);
}

// fragment shader
struct FragUniforms {
    time: f32,
    max_iter: f32,
    mousex: f32,
    mousey: f32,
    width: f32,
    height: f32,
    scale: f32,
};
@binding(0) @group(0) var<uniform> f_uniforms: FragUniforms;

@fragment
fn fs_main(@builtin(position) coord_in : vec4f) -> @location(0) vec4f {
    i_time = f_uniforms.time;
    max_iter = f_uniforms.max_iter;
    let mousex = f_uniforms.mousex;
    let mousey = f_uniforms.mousey;
    let w:f32 = f_uniforms.width;
    let h:f32 = f_uniforms.height;
    let scale = f_uniforms.scale;
    
    let z = vec2f(scale*(coord_in.x - 0.5*w)/w, -scale*(h/w)*(coord_in.y - 0.5*h)/h);

    let mouse = vec2f(7.0*(mousex/w - 0.5), 7.0*(mousey/h - 0.5));

    var ro = rotate(vec3f(0.0,0.0,2.5), mouse.x, mouse.y);
    var rd = -rotate(vec3f(z,1.0), mouse.x, mouse.y);
    let light = rotate(vec3f(0.0, 0.3, 0.77), mouse.x, mouse.y);

    var light_color = vec3f(0.8, 0.9, 1.0);
    var material:vec3f;
    var color:vec3f;
    var frag_color:vec4f;
    let eps = 0.002;
    var dist:f32;
   
    for( var n:f32 = 0.0; n < 100.0; n=n+1.0){
        dist = dist_estimate(ro);
        if(dist < eps) {break;}
        ro = ro + rd*dist*0.5;
    }
    
    if(backgr == true){
        color = vec3f(0.3, 0.8, 1.0) *(0.5 - 0.4*z.x);
        frag_color = vec4f(color, 1.0);
        return frag_color;
    }

    let norm = get_normal(ro, dist);
    material = hsv2rgb(vec3f(dot(ro, ro) - 0.27, 1.2, 1.0));
    
    let light_dir = normalize(light - rd);
    let shadow = soft_shadow(ro + 0.001*norm, light, 5.0);
    let ambient = 0.22;
    let diff = clamp(dot(light, norm), 0.0, 1.0)*shadow*0.9;
    let spec = pow(clamp(dot(norm, light_dir), 0.0, 1.0), 32.0)*shadow*1.8;
    color = light_color*(ambient + diff + spec)*material;
    color = pow(color, light_color);
    let fd = vec2f((6.0*coord_in.x - w)/h, 6.0*(coord_in.y-h)/h);
    color = color*(1.0-length(fd)*0.07);
    return vec4f(color, 1.0);
}
