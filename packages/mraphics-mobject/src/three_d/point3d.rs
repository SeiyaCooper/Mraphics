use mraphics_core::{
    BasicMaterial, Geometry, GeometryUpdater, GeometryView, Material, MaterialView, Mesh, MeshLike,
    RenderInstance, Renderable, Sphere,
};
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::wasm_bindgen;

#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub struct Point3D {
    pub radius: f32,

    #[cfg_attr(feature = "wasm", wasm_bindgen(skip))]
    pub identifier: usize,

    #[cfg_attr(feature = "wasm", wasm_bindgen(skip))]
    pub geometry: Sphere,

    #[cfg_attr(feature = "wasm", wasm_bindgen(skip))]
    pub material: BasicMaterial,
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
impl Point3D {
    #[cfg_attr(feature = "wasm", wasm_bindgen(js_name = "withRadius"))]
    pub fn with_radius(mut self, radius: f32) -> Self {
        self.radius = radius;
        self.geometry.radius = radius;
        self
    }
}

impl Default for Point3D {
    fn default() -> Self {
        Self {
            radius: 0.06,
            identifier: Mesh::<Sphere, BasicMaterial>::acquire_id(),
            geometry: Sphere {
                radius: 0.06,
                theta_segments: 8,
                phi_segments: 16,
                ..Default::default()
            },
            material: BasicMaterial::new(),
        }
    }
}

impl Renderable for Point3D {
    fn build_instance(&self) -> mraphics_core::RenderInstance {
        let mut instance = RenderInstance::new(self.identifier, &self.material);

        self.geometry.init_view(&mut instance.geometry);

        self.geometry.update_view(&mut instance.geometry);
        self.material.update_view(&mut instance.material);

        instance
    }

    fn identifier(&self) -> usize {
        self.identifier
    }

    fn init(&mut self) {
        self.geometry.init();
    }
}

impl MeshLike for Point3D {
    fn update_geometry_view(&self, view: &mut GeometryView) {
        self.geometry.update_view(view);
    }

    fn update_material_view(&self, view: &mut MaterialView) {
        self.material.update_view(view);
    }
}
