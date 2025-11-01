@group(0) @binding(0) var<uniform> view_mat: mat4x4<f32>;
@group(0) @binding(1) var<uniform> projection_mat: mat4x4<f32>;
@group(0) @binding(2) var<uniform> model_mat: mat4x4<f32>;

@group(1) @binding(0) var<storage, read> position: array<f32>;
 
@vertex
fn vs(@builtin(vertex_index) vertex_index: u32) -> @builtin(position) vec4f {
    let base_index = vertex_index * 3u;
    let position = vec3f(
        position[base_index],
        position[base_index + 1u],
        position[base_index + 2u]
    );
    return projection_mat * view_mat * model_mat * vec4f(position, 1.0);
}

@fragment
fn fs() -> @location(0) vec4f {
    return vec4f(0.3, 0.12, 0.72, 1.0);
}
