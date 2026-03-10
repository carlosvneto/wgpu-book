struct Output {
    @builtin(position) position : vec4f,
    @location(0) v_color : vec4f
};

// vertex shader

@vertex
fn vs_main(@builtin(vertex_index) in_vertex_index: u32) -> Output {    
    var pos : array<vec2f, 9> = array<vec2f, 9>(             
        vec2f(-0.63,  0.80),
        vec2f(-0.65,  0.20),
        vec2f(-0.20,  0.60),
        vec2f(-0.37, -0.07),
        vec2f( 0.05,  0.18),
        vec2f(-0.13, -0.40),
        vec2f( 0.30, -0.13),
        vec2f( 0.13, -0.64),
        vec2f( 0.70, -0.30)     
    );

    var color : array<vec3f, 9> = array<vec3f, 9>(             
        vec3f(1.0, 0.0, 0.0),
        vec3f(0.0, 1.0, 0.0),
        vec3f(0.0, 0.0, 1.0),
        vec3f(1.0, 0.0, 0.0),
        vec3f(0.0, 1.0, 0.0),
        vec3f(0.0, 0.0, 1.0),
        vec3f(1.0, 0.0, 0.0),
        vec3f(0.0, 1.0, 0.0),
        vec3f(0.0, 0.0, 1.0),  
    );

    var output: Output;
    output.position = vec4f(pos[in_vertex_index], 0.0, 1.0);
    output.v_color = vec4f(color[in_vertex_index], 1.0);
    return output;
}

// fragment shader

@fragment
fn fs_main(@location(0) v_color: vec4f) -> @location(0) vec4f {
    return v_color;
}
