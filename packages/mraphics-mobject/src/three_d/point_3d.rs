use mraphics_core::{
    BasicMaterial, Geometry, InstanceUpdater, Interpolatable, Material, Mesh, MeshLike,
    RenderInstance, Representable, Sphere, Transformable,
};
use nalgebra::Vector3;
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::wasm_bindgen;

#[derive(Clone)]
pub struct Point3DCenter {
    pub position: [f32; 3],
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub struct Point3D {
    pub radius: f32,

    #[cfg_attr(feature = "wasm", wasm_bindgen(skip))]
    pub center: Point3DCenter,

    #[cfg_attr(feature = "wasm", wasm_bindgen(skip))]
    pub identifier: usize,

    #[cfg_attr(feature = "wasm", wasm_bindgen(skip))]
    pub geometry: Sphere,

    #[cfg_attr(feature = "wasm", wasm_bindgen(skip))]
    pub material: BasicMaterial,
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
impl Point3D {
    #[cfg_attr(feature = "wasm", wasm_bindgen(constructor))]
    pub fn new() -> Self {
        Self::default()
    }

    #[cfg_attr(feature = "wasm", wasm_bindgen(js_name = "withRadius"))]
    pub fn with_radius(mut self, radius: f32) -> Self {
        self.radius = radius;
        self.geometry.radius = radius;
        self
    }
}

impl Point3D {
    pub fn with_center(mut self, center: [f32; 3]) -> Self {
        self.center = Point3DCenter { position: center };
        self
    }
}

impl Default for Point3D {
    fn default() -> Self {
        Self {
            radius: 0.06,
            center: Point3DCenter {
                position: [0.0, 0.0, 0.0],
            },
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

impl InstanceUpdater for Point3D {
    fn update_instance(&self, instance: &mut RenderInstance) {
        instance.move_to(&Vector3::from_column_slice(&self.center.position));
    }
}

impl MeshLike for Point3D {
    fn build_instance(&self) -> mraphics_core::RenderInstance {
        let mut instance = RenderInstance::new(self.identifier, &self.material);

        instance.move_to(&Vector3::from_column_slice(&self.center.position));

        self.geometry.init_view(&mut instance.geometry);

        self.geometry.update_view(&mut instance.geometry);
        self.material.update_view(&mut instance.material);

        instance
    }

    fn identifier(&self) -> usize {
        self.identifier
    }
}

impl Interpolatable for Point3DCenter {
    fn interpolate(&self, to: &Self, p: f32) -> Self {
        Self {
            position: self.position.interpolate(&to.position, p),
        }
    }
}

impl InstanceUpdater for Point3DCenter {
    fn update_instance(&self, instance: &mut RenderInstance) {
        instance.move_to(&Vector3::from_column_slice(&self.position));
    }
}

impl Representable for Point3D {
    type Intermediate = Point3DCenter;

    fn as_intermediate(&self) -> Self::Intermediate {
        self.center.clone()
    }

    fn update_from_intermediate(&mut self, repr: &Self::Intermediate) {
        self.center = repr.clone();
    }
}

impl Transformable for Point3D {
    fn apply_transform<Trans: Fn(&[f32; 3]) -> [f32; 3]>(
        &self,
        transform: Trans,
    ) -> Self::Intermediate {
        Point3DCenter {
            position: transform(&self.center.position),
        }
    }
}
