@group(0) @binding(0) var<uniform> view_mat: mat4x4<f32>;
@group(0) @binding(1) var<uniform> projection_mat: mat4x4<f32>;

@group(1) @binding(0) var<uniform> model_mat: mat4x4<f32>;
@group(1) @binding(5) var<uniform> thickness: f32;

@group(1) @binding(1) var<storage, read> position: array<vec4<f32>>;
@group(1) @binding(3) var<storage, read> previous: array<vec4<f32>>;
@group(1) @binding(2) var<storage, read> color: array<vec4<f32>>;
@group(1) @binding(4) var<storage, read> reverse: array<f32>;

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec4<f32>,
}

@vertex
fn vs(@builtin(vertex_index) vertex_index: u32) -> VertexOutput {
    let mvp = projection_mat * view_mat * model_mat;

    var projected = mvp * position[vertex_index];
    let previous_projected = mvp * previous[vertex_index];

    let aspect = projection_mat[1][1] / projection_mat[0][0];

    var screen = projected.xy / projected.w;
    screen.x = screen.x * aspect;

    var previous_screen = previous_projected.xy / previous_projected.w;
    previous_screen.x = previous_screen.x * aspect;

    let dir = normalize(screen - previous_screen);
    var normal = vec2<f32>(dir.y, -dir.x);

    normal = normal * thickness;
    normal.x = normal.x / aspect;

    var expand = normal * reverse[vertex_index];
    projected.x = projected.x + expand.x;
    projected.y = projected.y + expand.y;

    return VertexOutput(
        projected,
        color[vertex_index]
    );
}

@fragment
fn fs(vs_out: VertexOutput) -> @location(0) vec4f {
    return vs_out.color;
}