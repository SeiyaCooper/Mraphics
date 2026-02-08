@group(0) @binding(0) var<uniform> view_mat: mat4x4<f32>;
@group(0) @binding(1) var<uniform> projection_mat: mat4x4<f32>;

@group(2) @binding(0) var<uniform> color: vec4<f32>;

@group(1) @binding(0) var<uniform> model_mat: mat4x4<f32>;
@group(1) @binding(1) var<storage, read> position: array<vec4<f32>>;
 
@vertex
fn vs(@builtin(vertex_index) vertex_index: u32) -> @builtin(position) vec4f {
    return projection_mat * view_mat * model_mat * position[vertex_index];
}

@fragment
fn fs() -> @location(0) vec4f {
    return color;
}
