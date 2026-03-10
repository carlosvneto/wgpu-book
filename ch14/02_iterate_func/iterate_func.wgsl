// define iterated complex functions
fn c_func(z:vec2f, a:f32, select_id:i32) -> vec2f{
    var fz:vec2f = z;
   
    if (select_id == 0) {
        fz = c_mul(vec2f(a, a), c_log(c_mul(z,z)));
    } else if (select_id == 1){
        fz = c_div(c_log(c_mul(z,z)-vec2f(0.0, a)), c_exp(c_mul(z,z))-vec2f(a, 0.0));
    } else if (select_id == 2){
        fz = c_div(c_cos(z), c_sin(c_mul(z,z) - vec2f(0.5*a, 0.0)));
    } else if (select_id == 3){
        let f1 = c_inv(c_pow(z, 4.0) + vec2f(0.0, 0.1*a));
        fz = c_asinh(c_sin(f1));
    } else if (select_id == 4){
        let f1 = c_inv(c_pow(z, 6.0) + vec2f(0.0, 0.5*a));
        fz = c_log(c_sin(f1));
    } else if (select_id == 5){
        let f1 = c_mul(vec2f(0.0,1.0), c_cos(z));
        let f2 = c_sin(c_mul(z,z) - vec2f(a, 0.0));
        fz = c_div(f1, f2);
    } else if (select_id == 6){
        let f1 = c_cos(c_mul(vec2f(0.0,1.0), z));
        let f2 = c_sin(c_mul(z,z) - vec2f(a, 0.0));
        fz = c_div(f1, f2);
    } else if (select_id == 7){
        let f1 = c_tan(z);
        let f2 = c_sin(c_pow(z,8.0) - vec2f(0.5*a, 0.0));
        fz = c_div(f1, f2);
    } else if (select_id == 8){
        fz = c_inv(z) + c_div(c_mul(z,z), c_sin(c_pow(z,2.0) - vec2f(a, 0.0)));
    } else if (select_id == 9){
        fz = c_conj(z) + c_div(c_mul(z,z), c_sin(c_pow(z,2.0) - vec2f(2.0*a, 0.0)));
    } else if (select_id == 10){
        fz = c_sqrt(c_mul(vec2f(0.0,1.0), z)) + c_div(c_mul(z,z), c_sin(c_pow(z,2.0) - vec2f(2.0*a, 0.0)));
    } else {
        fz = c_mul(vec2f(a, a), c_log(c_mul(z,z)));
    }
    return fz;
}

// vertex shader
@vertex
fn vs_main(@builtin(vertex_index) in_vertex_index: u32) -> @builtin(position) vec4f {    
    var pos = array<vec2f, 4>(
        vec2f(-1.0, -1.0),
        vec2f( 1.0, -1.0),
        vec2f(-1.0,  1.0),
        vec2f( 1.0,  1.0),
    );
    return vec4f(pos[in_vertex_index], 0.0, 1.0);
}

// fragment shader
struct FragUniforms {
    a: f32,
    width: f32,
    height: f32,    
    select: f32,
    select_color: f32,                                   
};
@binding(0) @group(0) var<uniform> f_uniforms : FragUniforms;  

@fragment
fn fs_main(@builtin(position) coord_in : vec4f) -> @location(0) vec4f {
    let a = f_uniforms.a;       
    let w:f32 = f_uniforms.width;
    let h:f32 = f_uniforms.height;
    let aspect = w/h;
    var select_id:i32 = i32(f_uniforms.select);
    let color_id = i32(f_uniforms.select_color);
    let scale:f32 = 4.0;

    var z:vec2f = vec2f(scale*aspect*(coord_in.x - 0.5*w)/w, -scale*(coord_in.y - 0.5*h)/h);
    var iters:array<i32,11> = array<i32,11>(4,3,4,2,2,5,4,10,6,9,4);
    if(select_id >= 10) {select_id = 0;}
   
    var i:i32 = 0;
    loop {
        if(i >= iters[select_id]) {break;}      
        z = c_func(z, a, select_id);
        i = i + 1;
    }

    if (color_id > 0 && color_id < 12) { // colormaps
        return vec4f(colormap_to_rgb(z, color_id), 1.0);
    } else { // default hsv to rgb
        return vec4f(hsv_to_rgb(z), 1.0); 
    }
}
