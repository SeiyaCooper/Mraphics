use mraphics_core::{BasicMaterial, Mesh, MeshIndex, MeshLike, RenderInstance, Renderable, Sphere};
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::wasm_bindgen;

#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub struct Point3D {
    pub radius: f32,

    #[cfg_attr(feature = "wasm", wasm_bindgen(skip))]
    pub identifier: MeshIndex,

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
        RenderInstance::new(self.identifier.index().to_string(), &self.material)
    }

    fn identifier(&self) -> usize {
        self.identifier.index()
    }
}

impl MeshLike<Sphere, BasicMaterial> for Point3D {
    fn geometry(&self) -> &Sphere {
        &self.geometry
    }

    fn material(&self) -> &BasicMaterial {
        &self.material
    }
}
