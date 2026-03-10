var<private> pi:f32 = 3.14159265359;
var<private> cv:vec3f;
var<private> mcol:vec3f;
var<private> b_color:bool = false;
var<private> L:vec3f;
var<private> max_iter: i32;
var<private> i_time:f32;

fn de(p1: vec3f) -> f32{
    var dr = 1.0;
    var r = length(p1);
    var p = p1;
    for(var i:i32 = 0; i< max_iter; i=i+1){
        if(r>2.0) {break;}
        dr = 2.0*dr*r;
        let psi = abs((atan2(p.z, p.y)+pi/8.0)%(pi/4.0) - pi/8.0);
        p.y = cos(psi)*length(p.yz);
        p.z = sin(psi)*length(p.yz);
        let p2 = p*p;
        p = vec3f(vec2f(p2.x - p2.y, 2.0*p.x*p.y)*(1.0-p2.z/(p2.x+p2.y+p2.z)), 2.0*p.z*sqrt(p2.x+p2.y)) + cv;
        r = length(p);
        if(b_color == true && i == 3){mcol = p;}
    }
    return min(log(r)*r/max(dr, 1.0), 1.0);
}

fn rnd(c:vec2f) -> f32{
    return fract(sin(dot(vec2f(1.317, 19.753), c))*413.7972);
}

fn rnd_start(frag_coord:vec2f) -> f32{
    return 0.5+0.5*rnd(frag_coord.xy+vec2f(i_time*217.0, i_time*217.0));
}

fn shadao(ro:vec3f, rd:vec3f, px:f32, frag_coord:vec2f) ->f32{
    var res = 1.0;
    var t = 2.0*px*rnd_start(frag_coord);
    var d:f32;
    for(var i:i32 = 0; i<4; i=i+1){
        d = max(px, de(ro+rd*t)*1.5);
        t = t+d;
        res = min(res, d/t + t*0.1);
    }
    return res;
}

fn sky(rd: vec3f) -> vec3f{
    return vec3f(0.5 + 0.5*rd.y, 0.5 + 0.5*rd.y, 0.5 + 0.5*rd.y);
}

fn color(r1:vec3f, rd:vec3f, t:f32, px:f32, col:vec3f, b_fill:bool, frag_coord:vec2f) -> vec3f{
    var ro = r1+rd*t;
    b_color = true;
    var d = de(ro);
    b_color = false;
    let e = vec2f(px*t, 0.0);
    let dn = vec3f(de(ro-e.xyy), de(ro-e.yxy), de(ro-e.yyx));
    let dp = vec3f(de(ro+e.xyy), de(ro+e.yxy), de(ro+e.yyx));
    let N = (dp-dn)/(length(dp-vec3f(d,d,d))+ length(vec3f(d,d,d)-dn));
    let R = reflect(rd, N);
    let lc = vec3f(1.0, 0.9, 0.8);
    let sc = sqrt(abs(sin(mcol)));
    let rc = sky(R);
    var sh = clamp(shadao(ro, L, px*t, frag_coord)+0.2, 0.0, 1.0);
    sh = sh*(0.5+0.5*dot(N,L))*exp(-t*0.125);
    let scol = sh*lc*(sc+rc*pow(max(0.0, dot(R,L)), 4.0));
    
    if(b_fill){ d= d*0.05;}
    let col1 = mix(scol, col, clamp(d/(px*t), 0.0, 1.0));
    return col1;
}

fn look_at(p:vec3f) -> mat3x3<f32>{
    let p1 = normalize(p);
    let rt = normalize(cross(p1, vec3f(0.0, 1.0, 0.0)));
    return mat3x3<f32>(rt, cross(rt,p1), p1);
}

fn julia(t:f32) -> vec3f{
    let t1 = t%7.0;
    if(t1<1.0) { return vec3f(-0.8, 0.0, 0.0);}
    if(t1<2.0) { return vec3f(0.28, -0.5, 0.0);}
    if(t1<3.0) { return vec3f(-0.8, 1.0, -0.69);}
    if(t1<4.0) { return vec3f( 0.5, -0.84, -0.13);}
    if(t1<5.0) { return vec3f( 0.5, -0.34, 0.13);}
    if(t1<6.0) { return vec3f(-0.16, 0.657, 0.0);}

    return vec3f(0.0, 1.0, -1.0);
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
    max_iter = i32(f_uniforms.max_iter);  
    let w:f32 = f_uniforms.width;
    let h:f32 = f_uniforms.height;
    let scale = f_uniforms.scale;
    
    let px = 0.5/h;
    L = normalize(vec3f(0.4, 0.8, -0.6));
    var tim = 0.5*i_time;
   
    let ro = vec3f(cos(1.3*tim), sin(0.4*tim), sin(tim))*3.0;
    let rd = look_at(vec3f(-0.1, -0.1, -0.1)-ro)*normalize(vec3f(scale*(2.0*coord_in.x-w)/h, -scale*(2.0*coord_in.y-h)/h, 3.0));

    tim = tim*0.6;
    if(tim%15.0 < 7.0){
        cv = mix(julia(tim - 1.0), julia(tim), smoothstep(0.0, 1.0, fract(tim)*7.0));
    } else {
        cv = vec3f(-cos(tim), cos(tim)*abs(sin(tim*0.3)), -0.5*abs(sin(tim)));
    }

    var t = de(ro)*rnd_start(coord_in.xy);
    var d = 0.0;
    var od = 10.0;
    var edge = vec3f(-1.0, -1.0, -1.0);
    var b_grab = false;
    var col = sky(rd);
    
    for(var i:i32 = 0; i<78; i=i+1){
        t = t + 0.5*d;
        d = de(ro+rd*t);
        if(d>od){
            if(b_grab && od < px*t && edge.x < 0.0){
                edge = vec3f(edge.yz, t-od);
                b_grab = false;
            }
        } else {
            b_grab = true;
        }
        od = d;
        if(t > 10.0 || d < 0.00001) { break; }
    }
    
    var b_fill = false;
    d = d*0.05;
    if(d < px*t && t < 10.0){
        if (edge.x > 0.0) {edge = edge.zxy;}
        edge = vec3f(edge.yz, t);
        b_fill = true;
    }
    for(var i:i32 = 0; i < 3; i = i+1){
        if(edge.z > 0.0) {
            col = color(ro, rd, edge.z, px, col, b_fill, coord_in.xy);
            edge = edge.zxy;
            b_fill = false;
        }
    }

    return vec4f(col, 1.0);
}
