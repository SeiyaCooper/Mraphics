@group(0) @binding(0) var<uniform> view_mat: mat4x4<f32>;
@group(0) @binding(1) var<uniform> projection_mat: mat4x4<f32>;
@group(0) @binding(2) var<uniform> model_mat: mat4x4<f32>;

@group(1) @binding(0) var<storage, read> position: array<f32>;
@group(1) @binding(1) var<storage, read> color: array<f32>;

struct VertexOutput {
    @builtin(position) position: vec4f,
    @location(0) color: vec4f,
}
 
@vertex
fn vs(@builtin(vertex_index) vertex_index: u32) -> VertexOutput {
    let base_index = vertex_index * 3u;
    let position = vec3f(
        position[base_index],
        position[base_index + 1u],
        position[base_index + 2u]
    );

    let color = vec3f(
        color[base_index],
        color[base_index + 1u],
        color[base_index + 2u]
    );

    var out: VertexOutput;
    out.position = projection_mat * view_mat * model_mat * vec4f(position, 1.0);
    out.color = vec4f(color, 1.0);

    return out;
}

@fragment
fn fs(@location(0) color: vec4f) -> @location(0) vec4f {
    return color;
}
