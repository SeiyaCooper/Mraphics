use crate::{RenderProcess, render::GadgetData};

pub struct MaterialView {
    pub identifier: String,

    pub render_process: RenderProcess,

    pub uniforms: Vec<GadgetData>,
    pub storages: Vec<GadgetData>,
}

impl MaterialView {
    pub fn new(identifier: &str, render_process: RenderProcess) -> Self {
        Self {
            identifier: identifier.to_string(),

            render_process,
            uniforms: Vec::new(),
            storages: Vec::new(),
        }
    }

    pub fn with_uniforms(mut self, uniforms: Vec<GadgetData>) -> Self {
        self.uniforms = uniforms;
        self
    }

    pub fn with_storages(mut self, storages: Vec<GadgetData>) -> Self {
        self.storages = storages;
        self
    }

    pub fn reset(&mut self) {
        self.uniforms = Vec::new();
        self.storages = Vec::new();
    }
}

impl Default for MaterialView {
    fn default() -> Self {
        Self {
            identifier: String::from("mraphics default materila view"),
            render_process: RenderProcess::new(),
            uniforms: Vec::new(),
            storages: Vec::new(),
        }
    }
}

pub trait Material: Clone {
    fn identifier(&self) -> &str;
    fn init_view(&self, view: &mut MaterialView);
    fn update_view(&self, view: &mut MaterialView) {
        let _ = view;
    }
}
