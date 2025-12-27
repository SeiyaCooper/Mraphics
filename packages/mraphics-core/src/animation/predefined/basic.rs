use crate::{Action, Animation, RenderInstance, Renderable};
use nalgebra::{UnitQuaternion, UnitVector3, Vector3};
use std::{cell::RefCell, rc::Rc};

pub struct MeshAnimation<'res> {
    pub mesh_id: usize,
    pub on_update: Box<dyn FnMut(&mut RenderInstance, f32, f32) + 'res>,
    pub on_start: Box<dyn FnMut() + 'res>,
    pub on_stop: Box<dyn FnMut() + 'res>,
}

impl<'res> MeshAnimation<'res> {
    pub fn new<R: Renderable>(mesh: &R) -> Self {
        Self {
            mesh_id: mesh.identifier(),
            on_update: Box::new(|_, _, _| {}),
            on_start: Box::new(|| {}),
            on_stop: Box::new(|| {}),
        }
    }

    pub fn with_on_update<F: FnMut(&mut RenderInstance, f32, f32) + 'res>(
        mut self,
        closure: F,
    ) -> Self {
        self.on_update = Box::new(closure);
        self
    }

    pub fn with_on_start<F: FnMut() + 'res>(mut self, closure: F) -> Self {
        self.on_start = Box::new(closure);
        self
    }

    pub fn with_on_stop<F: FnMut() + 'res>(mut self, closure: F) -> Self {
        self.on_stop = Box::new(closure);
        self
    }
}

impl<'res> Animation<'res> for MeshAnimation<'res> {
    fn into_action(mut self, scene: std::rc::Rc<std::cell::RefCell<crate::Scene>>) -> Action<'res> {
        let mut out = Action::new();

        out.on_start = self.on_start;
        out.on_stop = self.on_stop;

        out.on_update = Box::new(move |progress, elapsed_time| {
            (self.on_update)(
                scene
                    .borrow_mut()
                    .acquire_instance_mut_unchecked(self.mesh_id),
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
    pub fn new<R: Renderable>(mesh: &R, axis: UnitVector3<f32>, angle_rad: f32) -> Self {
        Self {
            mesh_index: mesh.identifier(),
            axis,
            angle_rad,
        }
    }

    pub fn new_normalize<R: Renderable>(mesh: &R, axis: Vector3<f32>, angle_rad: f32) -> Self {
        Self {
            mesh_index: mesh.identifier(),
            axis: UnitVector3::new_normalize(axis),
            angle_rad,
        }
    }
}

impl Animation<'static> for RotateAxisAngle {
    fn into_action(self, scene: std::rc::Rc<std::cell::RefCell<crate::Scene>>) -> Action<'static> {
        let mut out = Action::new();
        let start_rotation = Rc::new(RefCell::new(UnitQuaternion::identity()));

        let scene_clone = scene.clone();
        let start_rotation_clone = start_rotation.clone();

        out.on_start = Box::new(move || {
            start_rotation_clone.borrow_mut().clone_from(
                scene_clone
                    .borrow()
                    .acquire_instance_unchecked(self.mesh_index)
                    .rotation(),
            );
        });
        out.on_update = Box::new(move |p, _| {
            scene
                .borrow_mut()
                .acquire_instance_mut_unchecked(self.mesh_index)
                .set_rotation(
                    &(UnitQuaternion::from_axis_angle(&self.axis, self.angle_rad * p)
                        * &*start_rotation.borrow()),
                );
        });

        out
    }
}
