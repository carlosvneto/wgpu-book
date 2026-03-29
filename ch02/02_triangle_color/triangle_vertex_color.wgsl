struct VOutput{   
    @location(0) v_color: vec4f,
    @builtin(position) position: vec4f,
};

@vertex
fn vs_main(@builtin(vertex_index) in_vertex_index: u32) -> VOutput {    
    var pos = array<vec2f, 3>(
        vec2f(0.0, 0.5),
        vec2f(-0.5,-0.5),
        vec2f(0.5,-0.5)
    );
    var color = array<vec3f, 3>(
        vec3f(1.0, 0.0, 0.0),
        vec3f(0.0, 1.0, 0.0),
        vec3f(0.0, 0.0, 1.0)
    );

    var out: VOutput;
    out.position = vec4f(pos[in_vertex_index], 0.0, 1.0);
    out.v_color = vec4f(color[in_vertex_index], 1.0);
    return out;
}

@fragment
fn fs_main(in: VOutput) -> @location(0) vec4f {
    return in.v_color;
}
