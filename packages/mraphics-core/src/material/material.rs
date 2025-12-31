use crate::render::GadgetData;

pub struct MaterialView {
    pub identifier: String,
    pub shader_code: String,

    pub uniforms: Vec<GadgetData>,
    pub attributes: Vec<GadgetData>,
}

impl MaterialView {
    pub fn new(identifier: &str) -> Self {
        Self {
            identifier: identifier.to_string(),
            shader_code: String::new(),
            uniforms: Vec::new(),
            attributes: Vec::new(),
        }
    }

    pub fn with_code(mut self, code: String) -> Self {
        self.shader_code = code;
        self
    }

    pub fn with_uniforms(mut self, uniforms: Vec<GadgetData>) -> Self {
        self.uniforms = uniforms;
        self
    }

    pub fn with_attributes(mut self, attributes: Vec<GadgetData>) -> Self {
        self.attributes = attributes;
        self
    }

    pub fn reset(&mut self) {
        self.uniforms = Vec::new();
        self.attributes = Vec::new();
    }
}

pub trait Material: Clone {
    fn identifier(&self) -> &str;
    fn shader_code(&self) -> &str;
    fn update_view(&self, view: &mut MaterialView);
}
