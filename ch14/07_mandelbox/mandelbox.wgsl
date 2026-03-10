// define global variables
var<private> e = vec2f(0.000035, -0.000035);
var<private> z:vec2f;
var<private> v:vec2f;
var<private> t:f32;
var<private> tt:f32;
var<private> b:f32;
var<private> g:f32;
var<private> g2:f32;
var<private> bb:f32;
var<private> np:vec3f;
var<private> bp:vec3f;
var<private> pp:vec3f;
var<private> po:vec3f;
var<private> no:vec3f;
var<private> al:vec3f;
var<private> ld:vec3f;

// define helper functions
fn bo(p:vec3f, r:vec3f) -> f32{
    let p1 = abs(p) - r;
    return max(max(p1.x, p1.y), p1.z);
}

fn r2(r:f32) -> mat2x2f{
    return mat2x2f(vec2f(cos(r), sin(r)), vec2f(-sin(r), cos(r)));
}

fn fb(p:vec3f, m:f32) -> vec2f {
    var p1 = p;
    p1.y = p1.y + 0.05*bb;
    var h = vec2f(bo(p1, vec3f(5.0, 1.0, 3.0)), 3.0);
    var t = vec2f(bo(p1, vec3f(5.0, 1.0, 3.0)), 3.0);
    t.x = max(t.x,-(length(p1) - 2.5));
    t.x = max(abs(t.x) - 0.2, p1.y - 0.4);
    h = vec2f(bo(p1,vec3f(5.0,1.0,3.0)),6.0);
    h.x = max(h.x,-(length(p1) - 2.5)); 
    h.x = max(abs(h.x) - 0.1, p1.y - 0.5);
    if(t.x >= h.x) {t = h;}
    h = vec2f(bo(p1+vec3f(0.0,0.4,0.0),vec3f(5.4,0.4,3.4)),m);
    h.x = max(h.x,-(length(p1) - 2.5));
    if(t.x >= h.x) {t = h;}
    h = vec2f(length(p1) - 2.0, m); 
    if(t.x >= h.x) {t = h;}
    t.x = 0.7*t.x;
    return t;
}

fn mp(p1:vec3f) -> vec2f{
    pp = p1;
    bp = p1;
    var p= p1;
    var pyz = p.yz*r2(sin(pp.x*0.3-tt*0.5)*0.4);
    p.y = pyz.x;
    p.z = pyz.y;
    bp.y = p.y;
    bp.z = p.z;
    pyz = p.yz*r2(1.57);
    p.y = pyz.x;
    p.z = pyz.y;
    b = sin(pp.x*0.2 + tt);
    bb = cos(pp.x*0.2 + tt);
    p.x = (p.x-tt*2.0)%10.0 - 5.0;
    var np = vec4f(p*0.4, 0.4); 
    for(var i:i32=0; i<4; i=i+1){
        np.x = abs(np.x) - 1.0;
        np.y = abs(np.y) - 1.2;
        np.z = abs(np.z);
        np.x = 2.0*clamp(np.x, 0.0, 2.0) - np.x;
        np.y = 2.0*clamp(np.y, 0.0, 0.0) - np.y;
        np.z = 2.0*clamp(np.z, 0.0, 4.3+bb) - np.z;
        np = np*(1.3)/clamp(dot(np.xyz,np.xyz), 0.1, 0.92);
    }
    var h = fb(abs(np.xyz)-vec3f(2.0,0.0,0.0),5.0);
    var t = fb(abs(np.xyz)-vec3f(2.0,0.0,0.0),5.0);
    t.x = t.x/np.w;
    t.x = max(t.x,bo(p, vec3f(5.0,5.0,10.0)));
    np = 0.5*np; 
    let np_yz =  np.yz * r2(0.785); 
    np.y = np_yz.x;
    np.z = np_yz.y; 
    np.y = np.y + 2.5;
    np.z = np.z + 2.5;
    h = fb(abs(np.xyz)-vec3f(0.0,4.5,0.0),7.0);
    h.x = max(h.x,-bo(p,vec3f(20.0,5.0,5.0)));
    h.x = h.x/(np.w*1.5);
    if(t.x >= h.x) {t = h;}
    h = vec2f(bo(np.xyz,vec3f(0.0,b*20.0,0.0)),6.0);
    h.x = h.x/(np.w*1.5);
    g2 = g2 + 0.1/(0.1*h.x*h.x*(1000.0-b*998.0));
    if(t.x >= h.x) {t = h;}
    h = vec2f(0.6*bp.y+sin(p.y*5.0)*0.03,6.0); 
    if(t.x >= h.x) {t = h;}
    h = vec2f(length(cos(bp.xyz*0.6+vec3f(tt,tt,0.0)))+0.003,6.0);
    g = g + 0.1/(0.1*h.x*h.x*4000.0);
    if(t.x >= h.x) {t = h;}
    return t;
}

fn tr(ro:vec3f, rd:vec3f) -> vec2f{
    var h = vec2f(0.1, 0.1);
    var t = vec2f(0.1, 0.1);
    for(var i:i32=0; i<100; i=i+1){
        h = mp(ro+rd*t.x);
        //if(h.x < 0.0001 || t.x > 40.0) { break; }
        if(h.x < 0.0001*(1.0+length(t)) || t.x > 40.0) { break; }
        t.x = t.x + h.x;
        t.y = h.y;
    }
    if(t.x > 40.0) {t.y = 0.0;}
    return t;
}

fn a(d:f32) -> f32{
    return clamp(mp(po+no*d).x/d, 0.0, 1.0);
}

fn s(d:f32) -> f32{
    return smoothstep(0.0,1.0,mp(po+ld*d).x/d);
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
    let i_time = f_uniforms.time;
    let w:f32 = f_uniforms.width;
    let h:f32 = f_uniforms.height;

    let uv = vec2f((coord_in.x/w - 0.5)*w/h, -(coord_in.y/h - 0.5));
    tt = i_time%62.8318;
    var ro = mix(vec3f(1.0,1.0,1.0),vec3f(-0.5,1.0,-1.0),ceil(sin(tt*0.5)))*vec3f(10.0,2.8+0.75*smoothstep(-1.5,1.5,1.5*cos(tt+0.2)),cos(tt*0.3)*3.1);
    var cw = normalize(vec3f(0.0,0.0,0.0)-ro); 
    var cu = normalize(cross(cw,normalize(vec3f(0.0,1.0,0.0))));
    var cv = normalize(cross(cu,cw));
    var rd = mat3x3<f32>(cu,cv,cw)*normalize(vec3f(uv, 0.5));
    ld = normalize(vec3f(0.2,0.4,-0.3));
    var co = vec3f(0.1,0.2,0.3)-length(uv)*0.1 - rd.y*0.2;
    var fo = vec3f(0.1,0.2,0.3)-length(uv)*0.1 - rd.y*0.2;
    z = tr(ro, rd);
    t = z.x;
    if(z.y > 0.0){
        po = ro + rd*t;
        no = normalize(e.xyy*mp(po+e.xyy).x+e.yyx*mp(po+e.yyx).x+e.yxy*mp(po+e.yxy).x+e.xxx*mp(po+e.xxx).x);
        al = mix(vec3f(0.1,0.2,0.4), vec3f(0.1,0.4,0.7), 0.5+0.5*sin(bp.y*7.0));
        if(z.y < 5.0) { al=vec3f(0.0,0.0,0.0);}
        if(z.y > 5.0) { al=vec3f(1.0,1.0,1.0);}
        if(z.y > 6.0) { al=mix(vec3f(1.0,0.5,0.0),vec3f(0.9,0.3,0.1), 0.5+0.5*sin(bp.y*7.0));}
        var dif = max(0.0, dot(no, ld));
        var fr = pow(1.0 + dot(no, rd), 4.0);
        var sp = pow(max(dot(reflect(-ld,no),-rd),0.0),40.0);
        co = mix(sp+mix(vec3f(0.8,0.8,0.8),vec3f(1.0,1.0,1.0),abs(rd))*al*(a(0.1)*a(0.2)+0.2)*(dif+s(2.0)),fo,min(fr,0.2));
        co = mix(fo,co,exp(-0.0003*t*t*t)); 
    }

    return vec4f(pow(co+g*0.2+g2*mix(vec3f(1.0,0.5,0.0),vec3f(0.9,0.3,0.1), 0.5+0.5*sin(bp.y*3.0)),vec3f(0.65, 0.65, 0.65)),1.0);
}
