use crate::render::GadgetData;

pub trait Material: MaterialView {
    fn identifier(&self) -> &'static str;

    fn shader_code(&self) -> String;
}

pub trait MaterialView {
    fn uniforms(&self) -> &Vec<GadgetData>;
    fn uniforms_mut(&mut self) -> &mut Vec<GadgetData>;

    fn attributes(&self) -> &Vec<GadgetData>;
    fn attributes_mut(&mut self) -> &mut Vec<GadgetData>;
}

#[macro_export]
macro_rules! impl_material {
    ($type:ty) => {
        impl $crate::material::MaterialView for $type {
            fn attributes(&self) -> &Vec<crate::render::GadgetData> {
                &self.attributes
            }

            fn attributes_mut(&mut self) -> &mut Vec<crate::render::GadgetData> {
                &mut self.attributes
            }

            fn uniforms(&self) -> &Vec<crate::render::GadgetData> {
                &self.uniforms
            }

            fn uniforms_mut(&mut self) -> &mut Vec<crate::render::GadgetData> {
                &mut self.uniforms
            }
        }
    };
}
