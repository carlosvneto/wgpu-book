// vertex shader

struct Uniforms {   
    model_mat : mat4x4f,
    view_project_mat : mat4x4f,
    normal_mat : mat4x4f,            
};
@binding(0) @group(0) var<uniform> uniforms : Uniforms;

struct Input {
    @location(0) pos : vec4f,
    @location(1) normal : vec4f,
    @location(2) uv : vec2f,
    @location(3) color : vec4f,
};

struct Output {
    @builtin(position) position : vec4f,
    @location(0) v_position : vec4f,
    @location(1) v_normal : vec4f,
    @location(2) v_uv : vec2f,
    @location(3) v_color : vec4f,
};

@vertex
fn vs_main(in: Input) -> Output {    
    var output: Output;            
    let m_position:vec4f = uniforms.model_mat * in.pos; 
    output.v_position = m_position;                  
    output.v_normal =  uniforms.normal_mat * in.normal;
    output.v_uv =  in.uv;
    output.position = uniforms.view_project_mat * m_position;
    output.v_color = in.color;    
    return output;
}

// fragment shader

struct FragUniforms {
    light_position : vec4f,
    eye_position : vec4f,
};
@binding(1) @group(0) var<uniform> frag_uniforms : FragUniforms;

struct LightUniforms {  
    specular_color : vec4f,
    ambient_intensity: f32,
    diffuse_intensity :f32,
    specular_intensity: f32,
    specular_shininess: f32,
    is_two_side: i32,
};
@binding(2) @group(0) var<uniform> light_uniforms : LightUniforms;

@binding(0) @group(1) var texture_data : texture_2d<f32>;
@binding(1) @group(1) var texture_sampler : sampler;

@fragment
fn fs_main(in:Output) -> @location(0) vec4f {
    let texture_color:vec4f = textureSample(texture_data, texture_sampler, in.v_uv);

    let N:vec3f = normalize(in.v_normal.xyz);                
    let L:vec3f = normalize(frag_uniforms.light_position.xyz - in.v_position.xyz);     
    let V:vec3f = normalize(frag_uniforms.eye_position.xyz - in.v_position.xyz);          
    let H:vec3f = normalize(L + V);
    
    // front side
    var diffuse: f32 = light_uniforms.diffuse_intensity * max(dot(N, L), 0.0);
    var specular: f32 = light_uniforms.specular_intensity * pow(max(dot(N, H),0.0), light_uniforms.specular_shininess);

    // back side
    var is_two_side: i32 = light_uniforms.is_two_side;
    if(is_two_side == 1) {
        diffuse = diffuse + light_uniforms.diffuse_intensity * max(dot(-N, L), 0.0);
        specular = specular + light_uniforms.specular_intensity * pow(max(dot(-N, H),0.0), light_uniforms.specular_shininess);
    }    
   
    let ambient: f32 = light_uniforms.ambient_intensity;               
    let final_color:vec3f = (texture_color.rgb + in.v_color.xyz)*(ambient + diffuse) + light_uniforms.specular_color.xyz * specular; 
    return vec4f(final_color, 1.0);
}
