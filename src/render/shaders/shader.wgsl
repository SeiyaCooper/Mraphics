@vertex
fn vs(@builtin(vertex_index) vertex_index: u32) -> @builtin(position) vec4f {
    let vertices = array(
        vec2f(0.5, 0.5),
        vec2f(-0.5, 0.5),
        vec2f(0.0, -0.5),
    );
    return vec4f(vertices[vertex_index], 0.0, 1.0);
}

@fragment
fn fs() -> @location(0) vec4f {
    return vec4f(0.3, 0.12, 0.72, 1.0);
}
