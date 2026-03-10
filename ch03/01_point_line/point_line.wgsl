@vertex
fn vs_main(@builtin(vertex_index) in_vertex_index: u32) -> @builtin(position) vec4f {    
    var pos = array<vec2f, 6>(
        vec2f(-0.5,  0.7),
        vec2f( 0.3,  0.6),
        vec2f( 0.5,  0.3),
        vec2f( 0.4, -0.5),
        vec2f(-0.4, -0.4),
        vec2f(-0.3,  0.2)
    );
    return vec4f(pos[in_vertex_index], 0.0, 1.0);
}

@fragment
fn fs_main() -> @location(0) vec4f {
    return vec4f(1.0, 1.0, 0.0, 1.0);
}
