@group(0) @binding(0) var<uniform> view_mat: mat4x4<f32>;
@group(0) @binding(1) var<uniform> projection_mat: mat4x4<f32>;

@group(1) @binding(0) var<uniform> model_mat: mat4x4<f32>;
@group(1) @binding(1) var<storage, read> position: array<vec4<f32>>;
@group(1) @binding(2) var<storage, read> color: array<vec4<f32>>;

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec4<f32>,
}
 
@vertex
fn vs(@builtin(vertex_index) vertex_index: u32) -> VertexOutput {
    var out: VertexOutput;

    out.position = projection_mat * view_mat * model_mat * position[vertex_index];
    out.color = color[vertex_index];

    return out;
}

@fragment
fn fs(vs_out: VertexOutput) -> @location(0) vec4f {
    return vs_out.color;
}
