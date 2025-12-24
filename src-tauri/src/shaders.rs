pub const MIX_SHADER: &str = r#"
struct MixUniforms {
    factor: f32,
    p1: f32,
    p2: f32,
    p3: f32,
};

@group(0) @binding(0) var tex_a: texture_2d<f32>;
@group(0) @binding(1) var tex_b: texture_2d<f32>;
@group(0) @binding(2) var samp: sampler;
@group(0) @binding(3) var<uniform> uniforms: MixUniforms;

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
};

@vertex
fn vs_main(@builtin(vertex_index) in_vertex_index: u32) -> VertexOutput {
    var out: VertexOutput;
    // Full screen triangle
    var positions = array<vec2<f32>, 3>(
        vec2<f32>(-1.0, 1.0),
        vec2<f32>(3.0, 1.0),
        vec2<f32>(-1.0, -3.0)
    );
    var uvs = array<vec2<f32>, 3>(
        vec2<f32>(0.0, 0.0),
        vec2<f32>(2.0, 0.0),
        vec2<f32>(0.0, 2.0)
    );
    
    let pos = positions[in_vertex_index];
    out.position = vec4<f32>(pos, 0.0, 1.0);
    out.uv = uvs[in_vertex_index];
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let color_a = textureSample(tex_a, samp, in.uv);
    let color_b = textureSample(tex_b, samp, in.uv);
    
    return mix(color_a, color_b, uniforms.factor);
}
"#;
