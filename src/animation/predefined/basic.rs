use std::{cell::RefCell, rc::Rc};

use crate::{
    animation::{Action, Animation},
    geometry::Mesh,
};
use nalgebra::{UnitQuaternion, UnitVector3, Vector3};

pub struct MeshAnimation {
    pub mesh_index: usize,
    pub on_update: Box<dyn FnMut(&mut Mesh, f32, f32)>,
    pub on_start: Box<dyn FnMut()>,
    pub on_stop: Box<dyn FnMut()>,
}

impl MeshAnimation {
    pub fn new(mesh_index: usize) -> Self {
        Self {
            mesh_index,
            on_update: Box::new(|_, _, _| {}),
            on_start: Box::new(|| {}),
            on_stop: Box::new(|| {}),
        }
    }

    pub fn with_on_update<F: FnMut(&mut Mesh, f32, f32) + 'static>(mut self, closure: F) -> Self {
        self.on_update = Box::new(closure);
        self
    }

    pub fn with_on_start<F: FnMut() + 'static>(mut self, closure: F) -> Self {
        self.on_start = Box::new(closure);
        self
    }

    pub fn with_on_stop<F: FnMut() + 'static>(mut self, closure: F) -> Self {
        self.on_stop = Box::new(closure);
        self
    }
}

impl Animation for MeshAnimation {
    fn into_action(mut self, scene: std::rc::Rc<std::cell::RefCell<crate::Scene>>) -> Action {
        let mut out = Action::new();

        out.on_start = self.on_start;
        out.on_stop = self.on_stop;

        out.on_update = Box::new(move |progress, elapsed_time| {
            (self.on_update)(
                scene.borrow_mut().get_mesh_mut(self.mesh_index),
                progress,
                elapsed_time,
            )
        });

        out
    }
}

pub struct RotateAxisAngle {
    pub mesh_index: usize,
    pub axis: UnitVector3<f32>,
    pub angle_rad: f32,
}

impl RotateAxisAngle {
    pub fn new(mesh_index: usize, axis: UnitVector3<f32>, angle_rad: f32) -> Self {
        Self {
            mesh_index,
            axis,
            angle_rad,
        }
    }

    pub fn new_normalize(mesh_index: usize, axis: Vector3<f32>, angle_rad: f32) -> Self {
        Self {
            mesh_index,
            axis: UnitVector3::new_normalize(axis),
            angle_rad,
        }
    }
}

impl Animation for RotateAxisAngle {
    fn into_action(self, scene: std::rc::Rc<std::cell::RefCell<crate::Scene>>) -> Action {
        let mut out = Action::new();
        let start_rotation = Rc::new(RefCell::new(UnitQuaternion::identity()));

        let scene_clone = scene.clone();
        let start_rotation_clone = start_rotation.clone();

        out.on_start = Box::new(move || {
            start_rotation_clone
                .borrow_mut()
                .clone_from(scene_clone.borrow().get_mesh(self.mesh_index).rotation());
        });
        out.on_update = Box::new(move |p, _| {
            scene
                .borrow_mut()
                .get_mesh_mut(self.mesh_index)
                .set_rotation(
                    &(UnitQuaternion::from_axis_angle(&self.axis, self.angle_rad * p)
                        * &*start_rotation.borrow()),
                );
        });

        out
    }
}
