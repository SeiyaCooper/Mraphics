use crate::{geometry::GeometryView, material::Material};
use nalgebra::{Isometry3, Matrix4, Translation3, UnitQuaternion, UnitVector3, Vector3};

pub struct Mesh {
    pub children: Vec<Mesh>,
    pub geometry: Box<dyn GeometryView>,
    pub material: Box<dyn Material>,

    scale: Vector3<f32>,
    isometry: Isometry3<f32>,
    matrix: Matrix4<f32>,
}

impl Mesh {
    pub fn new<G: GeometryView + 'static, M: Material + 'static>(geometry: G, material: M) -> Self {
        Self {
            children: Vec::new(),
            geometry: Box::new(geometry),
            material: Box::new(material),

            scale: Vector3::new(1.0, 1.0, 1.0),
            isometry: Isometry3::new(Vector3::zeros(), Vector3::zeros()),
            matrix: Matrix4::identity(),
        }
    }

    pub fn add_child(&mut self, child: Mesh) {
        self.children.push(child);
    }

    pub fn traverse<F: Fn(&Mesh)>(&self, callback: &F) {
        callback(self);

        for child in &self.children {
            child.traverse(callback);
        }
    }

    pub fn traverse_mut<F: FnMut(&mut Mesh)>(&mut self, callback: &mut F) {
        callback(self);

        for child in &mut self.children {
            child.traverse_mut(callback);
        }
    }

    pub fn matrix(&self) -> &Matrix4<f32> {
        &self.matrix
    }

    pub fn rotation(&self) -> &UnitQuaternion<f32> {
        &self.isometry.rotation
    }

    pub fn set_rotation(&mut self, rotation: &UnitQuaternion<f32>) {
        self.isometry.rotation.clone_from(rotation);
        self.update_matrix();
    }

    pub fn rotate_x(&mut self, angle_rad: f32) {
        self.isometry.rotation =
            UnitQuaternion::from_axis_angle(&UnitVector3::new_normalize(Vector3::x()), angle_rad)
                * self.isometry.rotation;
        self.update_matrix();
    }

    pub fn rotate_y(&mut self, angle_rad: f32) {
        self.isometry.rotation =
            UnitQuaternion::from_axis_angle(&UnitVector3::new_normalize(Vector3::y()), angle_rad)
                * self.isometry.rotation;
        self.update_matrix();
    }

    pub fn rotate_z(&mut self, angle_rad: f32) {
        self.isometry.rotation =
            UnitQuaternion::from_axis_angle(&UnitVector3::new_normalize(Vector3::z()), angle_rad)
                * self.isometry.rotation;
        self.update_matrix();
    }

    pub fn translation(&self) -> &Translation3<f32> {
        &self.isometry.translation
    }

    pub fn scale(&self) -> &Vector3<f32> {
        &self.scale
    }

    pub fn scale_by(&mut self, factor: &Vector3<f32>) {
        self.scale.component_mul_assign(factor);
        self.update_matrix();
    }

    pub fn scale_to(&mut self, scale: &Vector3<f32>) {
        self.scale.copy_from(scale);
        self.update_matrix();
    }

    fn update_matrix(&mut self) {
        self.matrix = self.isometry.to_homogeneous() * Matrix4::new_nonuniform_scaling(&self.scale);
    }
}
