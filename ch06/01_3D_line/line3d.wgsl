struct Uniforms {
    mvpMatrix : mat4x4f,
};
@binding(0) @group(0) var<uniform> uniforms : Uniforms;

@vertex
fn vs_main(@location(0) pos: vec4f) ->  @builtin(position) vec4f {
    return uniforms.mvpMatrix * pos;                
}

@fragment
fn fs_main() -> @location(0) vec4f {
    return vec4f(1.0, 1.0, 0.0, 1.0);            
}
