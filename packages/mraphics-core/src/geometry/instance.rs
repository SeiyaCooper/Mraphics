use crate::{
    GeometryView, Material, MaterialView, MraphicsID,
    constants::{MODEL_MAT_LABEL, PrimitiveTopology},
};
use nalgebra::{Isometry3, Matrix4, Translation3, UnitQuaternion, UnitVector3, Vector3};

#[derive(Debug)]
pub struct RenderInstance {
    pub identifier: MraphicsID,
    pub geometry: GeometryView,
    pub material: MaterialView,

    pub children: Vec<RenderInstance>,

    pub topology: PrimitiveTopology,

    scale: Vector3<f32>,
    isometry: Isometry3<f32>,
    matrix: Matrix4<f32>,
}

impl RenderInstance {
    pub fn new<M: Material>(identifier: MraphicsID, material: &M) -> Self {
        Self {
            identifier,
            geometry: GeometryView::new(),
            material: MaterialView::new(material.identifier())
                .with_code(material.shader_code().to_string()),

            children: Vec::new(),

            topology: PrimitiveTopology::TriangleList,

            scale: Vector3::new(1.0, 1.0, 1.0),
            isometry: Isometry3::new(Vector3::zeros(), Vector3::zeros()),
            matrix: Matrix4::identity(),
        }
    }

    pub fn add_child(&mut self, child: RenderInstance) {
        self.children.push(child);
    }

    pub fn remove_child(&mut self, identifier: MraphicsID) {
        self.children.retain(|child| child.identifier != identifier);
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

    pub fn move_to(&mut self, position: &Vector3<f32>) {
        self.isometry.translation.vector = *position;
        self.update_matrix();
    }

    pub fn move_by(&mut self, offset: &Vector3<f32>) {
        self.isometry.translation.vector += offset;
        self.update_matrix();
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

    pub fn sync_matrix_data(&mut self) {
        self.geometry
            .set_uniform(
                MODEL_MAT_LABEL,
                bytemuck::cast_slice(self.matrix.as_slice()).to_vec(),
            )
            .unwrap();
    }

    fn update_matrix(&mut self) {
        self.matrix = self.isometry.to_homogeneous() * Matrix4::new_nonuniform_scaling(&self.scale);
        self.sync_matrix_data();
    }
}

pub trait InstanceUpdater {
    fn update_instance(&self, instance: &mut RenderInstance);
}
