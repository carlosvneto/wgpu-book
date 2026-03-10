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
    max_iter: f32,
    cx: f32,
    cy: f32,
    width: f32,
    height: f32,
    select_color: f32,
};
@binding(0) @group(0) var<uniform> f_uniforms : FragUniforms;

@fragment
fn fs_main(@builtin(position) coord_in : vec4f) -> @location(0) vec4f {
    let pi:f32 = 3.1415926;
    let max_iter = i32(f_uniforms.max_iter);    
    let cx = f_uniforms.cx;
    let cy = f_uniforms.cy;
    let w:f32 = f_uniforms.width;
    let h:f32 = f_uniforms.height;
    let color_id = i32(f_uniforms.select_color);
    let aspect = w/h;
    let scale: f32 = 2.2;
   
    var x: f32 = scale*aspect*(coord_in.x - 0.5*w)/w + cx;
    var y: f32 = -scale*(coord_in.y - 0.5*h)/h - cy;

    var z: vec2f = vec2f(0.0, 0.0);
    var i:i32 = 0;

    loop {
        if(i >= max_iter) {break;}  
        z = vec2f(z.x*z.x - z.y*z.y + x, 2.0*z.x*z.y + y);
        i = i + 1;
        if(z.x*z.x + z.y*z.y > 4.0) {break;}
    }

    var v: f32;
    if(color_id == 1){
        v = fract(log2(f32(i)));
        v = clamp(0.0, v, 1.0);
        return vec4f(v, v*v, v*v*v, 1.0);
    } else {
        v = f32(i)/f_uniforms.max_iter;
        return vec4f(v, 0.5*(sin(5.0*pi*v*v)+1.0), 0.5*(cos(5.0*pi*v)+1.0), 1.0);
    }
}
