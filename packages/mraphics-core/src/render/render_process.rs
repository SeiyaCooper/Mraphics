#[derive(Debug, Clone, Copy)]
pub enum PassContext {
    Render,
    Compute { workgroup_size: (u32, u32, u32) },
}

/// Describes a single pass in the render process.
/// For a Render pass, shader_code should contain vertex + fragment shaders.
/// For a Compute pass, shader_code should contain a compute shader.
#[derive(Debug, Clone)]
pub struct Pass {
    pub shader_code: String,
    pub context: PassContext,
}

impl Pass {
    pub fn render(shader_code: &str) -> Self {
        Self {
            shader_code: shader_code.to_string(),
            context: PassContext::Render,
        }
    }

    pub fn compute(shader_code: &str, workgroup_size: (u32, u32, u32)) -> Self {
        Self {
            shader_code: shader_code.to_string(),
            context: PassContext::Compute { workgroup_size },
        }
    }
}

/// A user-defined sequence of render and/or compute passes.
pub struct RenderProcess {
    pub passes: Vec<Pass>,
}

impl RenderProcess {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn queue_pass(&mut self, pass: Pass) {
        self.passes.push(pass);
    }
}

impl Default for RenderProcess {
    fn default() -> Self {
        Self { passes: Vec::new() }
    }
}
