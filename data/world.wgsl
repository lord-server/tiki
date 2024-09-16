struct VertexOutput {
    @builtin(position) position: vec4f,
    @location(0) texcoord: vec2f,
    @location(1) normal: vec3f,
};

@vertex
fn vs_main(
    @location(0) position: vec3f,
    @location(1) normal: vec3f,
    @location(2) texcoord: vec2f,
) -> VertexOutput {
    var result: VertexOutput;
    result.texcoord = texcoord;
    result.position = vec4<f32>(position, 1.0);
    result.normal = normal;
    return result;
}

@fragment
fn fs_main(vertex: VertexOutput) -> @location(0) vec4f {
    return vec4<f32>(vertex.texcoord.x, vertex.texcoord.y, 1.0, 1.0);
}
