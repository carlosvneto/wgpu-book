// vertex shader
struct VertexUniforms {
    screenDimensions : vec2f,
    particleSize : f32,
};
@binding(0) @group(0) var<uniform> uniforms : VertexUniforms;

struct Input {
    @location(0) vertexPosition : vec2f,
    @location(1) color : vec4f,
    @location(2) position : vec4f,
};

struct Output {
    @builtin(position) Position : vec4f,
    @location(0) vColor : vec4f,
};

@vertex
fn vs_main(input: Input) -> Output {
    var output: Output;
    output.vColor = input.color;
    output.Position = vec4f(
        input.vertexPosition* uniforms.particleSize / uniforms.screenDimensions + input.position.xy,
        input.position.z,
        1.0
    );
    return output;
}

// fragment shader
@fragment
fn fs_main( @location(0) vColor : vec4f) -> @location(0) vec4f {
    return vec4f(vColor.rgb * vColor.a, vColor.a);
}

// compute shader
struct PositionVelocity {
    pv: array<vec4f>,
};

struct Mass {
    mass1Position : vec4f,
    mass2Position : vec4f,
    mass3Position : vec4f,
    mass1Factor : f32,
    mass2Factor : f32,
    mass3Factor : f32,
};

@binding(0) @group(0) var<storage, read> positionIn : PositionVelocity;
@binding(1) @group(0) var<storage, read> velocityIn : PositionVelocity;
@binding(2) @group(0) var<storage, read_write> positionOut : PositionVelocity;
@binding(3) @group(0) var<storage, read_write> velocityOut : PositionVelocity;
@binding(4) @group(0) var<uniform> mass : Mass;

@compute @workgroup_size(64)
fn cs_main(@builtin(global_invocation_id) GlobalInvocationID : vec3<u32>) {
    var index:u32 =  GlobalInvocationID.x;
    var position:vec3f = positionIn.pv[index].xyz;
    var velocity:vec3f = velocityIn.pv[index].xyz;
    var massVec:vec3f = mass.mass1Position.xyz-position;
    var massDist2:f32 = max(0.01, dot(massVec, massVec));               
    var acceleration:vec3f = mass.mass1Factor/massDist2 * normalize(massVec);
    massVec = mass.mass2Position.xyz-position;
    massDist2 = max(0.01, dot(massVec, massVec));                
    acceleration = acceleration + mass.mass2Factor/massDist2 * normalize(massVec);
    massVec = mass.mass3Position.xyz-position;
    massDist2 = max(0.01, dot(massVec, massVec));               
    acceleration = acceleration + mass.mass3Factor/massDist2 * normalize(massVec);
    velocity = velocity + acceleration;
    velocity = 0.995 * velocity;             

    //write back  
    positionOut.pv[index] = vec4f(position + velocity, 1.0);               
    velocityOut.pv[index] = vec4f(velocity, 1.0);                                 
}
