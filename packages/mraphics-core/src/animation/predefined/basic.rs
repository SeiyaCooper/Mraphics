use crate::{Action, Animation, MeshHandle, MeshLike, MeshPool, RenderInstance, Scene};
use nalgebra::{UnitQuaternion, UnitVector3, Vector3};
use std::{cell::RefCell, rc::Rc};

pub struct MeshAnimation<'res, M: MeshLike + 'static> {
    pub mesh_id: usize,
    pub on_update: Box<dyn FnMut(&mut M, &mut RenderInstance, f32, f32) + 'res>,
    pub on_start: Box<dyn FnMut() + 'res>,
    pub on_stop: Box<dyn FnMut() + 'res>,
}

impl<'res, M: MeshLike + 'static> MeshAnimation<'res, M> {
    pub fn new(mesh_handle: &MeshHandle<M>) -> Self {
        Self {
            mesh_id: mesh_handle.id,
            on_update: Box::new(|_, _, _, _| {}),
            on_start: Box::new(|| {}),
            on_stop: Box::new(|| {}),
        }
    }

    pub fn with_on_update<F: FnMut(&mut M, &mut RenderInstance, f32, f32) + 'res>(
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

impl<'res, M: MeshLike + 'static> Animation<'res> for MeshAnimation<'res, M> {
    fn into_action(
        mut self,
        mesh_pool: Rc<RefCell<MeshPool>>,
        scene: Rc<RefCell<Scene>>,
    ) -> Action<'res> {
        let mut out = Action::new();

        out.on_start = self.on_start;
        out.on_stop = self.on_stop;

        out.on_update = Box::new(move |progress, elapsed_time| {
            (self.on_update)(
                mesh_pool
                    .borrow_mut()
                    .acquire_mesh_mut_unchecked(self.mesh_id),
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

/// Rotates the mesh around a given axis by a given angle.
pub struct RotateAxisAngle {
    /// The unique identifier of the mesh to animate.
    pub mesh_id: usize,

    /// The axis of rotation, normalized to unit length.
    pub axis: UnitVector3<f32>,

    /// The rotation angle in radians for this animation.
    pub angle_rad: f32,
}

impl RotateAxisAngle {
    pub fn new<M: MeshLike + 'static>(
        mesh_handle: &MeshHandle<M>,
        axis: UnitVector3<f32>,
        angle_rad: f32,
    ) -> Self {
        Self {
            mesh_id: mesh_handle.id,
            axis,
            angle_rad,
        }
    }

    pub fn new_normalize<M: MeshLike + 'static>(
        mesh_handle: MeshHandle<M>,
        axis: Vector3<f32>,
        angle_rad: f32,
    ) -> Self {
        Self {
            mesh_id: mesh_handle.id,
            axis: UnitVector3::new_normalize(axis),
            angle_rad,
        }
    }
}

impl Animation<'static> for RotateAxisAngle {
    fn into_action(
        self,
        _mesh_pool: Rc<RefCell<MeshPool>>,
        scene: Rc<RefCell<Scene>>,
    ) -> Action<'static> {
        let mut out = Action::new();
        let start_rotation = Rc::new(RefCell::new(UnitQuaternion::identity()));

        let scene_clone = scene.clone();
        let start_rotation_clone = start_rotation.clone();

        out.on_start = Box::new(move || {
            start_rotation_clone.borrow_mut().clone_from(
                scene_clone
                    .borrow()
                    .acquire_instance_unchecked(self.mesh_id)
                    .rotation(),
            );
        });
        out.on_update = Box::new(move |p, _| {
            scene
                .borrow_mut()
                .acquire_instance_mut_unchecked(self.mesh_id)
                .set_rotation(
                    &(UnitQuaternion::from_axis_angle(&self.axis, self.angle_rad * p)
                        * &*start_rotation.borrow()),
                );
        });

        out
    }
}

/// Shifts the mesh to the specific place
pub struct MoveTo {
    pub mesh_id: usize,
    pub target_place: Vector3<f32>,
}

impl MoveTo {
    pub fn new<M: MeshLike + 'static>(
        mesh_handle: &MeshHandle<M>,
        target_place: Vector3<f32>,
    ) -> Self {
        Self {
            mesh_id: mesh_handle.id,
            target_place,
        }
    }
}

impl Animation<'static> for MoveTo {
    fn into_action(
        self,
        _mesh_pool: Rc<RefCell<MeshPool>>,
        scene: Rc<RefCell<Scene>>,
    ) -> Action<'static> {
        let mut out = Action::new();
        let start_place = Rc::new(RefCell::new(Vector3::default()));

        let scene_clone = scene.clone();
        let start_place_clone = start_place.clone();

        out.on_start = Box::new(move || {
            start_place_clone.borrow_mut().clone_from(
                &scene_clone
                    .borrow()
                    .acquire_instance_unchecked(self.mesh_id)
                    .translation()
                    .vector,
            );
        });
        out.on_update = Box::new(move |p, _| {
            scene
                .borrow_mut()
                .acquire_instance_mut_unchecked(self.mesh_id)
                .move_to(
                    &(*start_place.borrow() + &((self.target_place - *start_place.borrow()) * p)),
                );
        });

        out
    }
}

pub struct ScaleTo {
    pub mesh_id: usize,
    pub target_scale: Vector3<f32>,
}

impl ScaleTo {
    pub fn new<M: MeshLike + 'static>(
        mesh_handle: MeshHandle<M>,
        target_scale: Vector3<f32>,
    ) -> Self {
        Self {
            mesh_id: mesh_handle.id,
            target_scale,
        }
    }
}

impl Animation<'static> for ScaleTo {
    fn into_action(
        self,
        _mesh_pool: Rc<RefCell<MeshPool>>,
        scene: Rc<RefCell<Scene>>,
    ) -> Action<'static> {
        let mut out = Action::new();
        let start_scale = Rc::new(RefCell::new(Vector3::default()));

        let scene_clone = scene.clone();
        let start_scale_clone = start_scale.clone();

        out.on_start = Box::new(move || {
            start_scale_clone.borrow_mut().clone_from(
                scene_clone
                    .borrow()
                    .acquire_instance_unchecked(self.mesh_id)
                    .scale(),
            );
        });
        out.on_update = Box::new(move |p, _| {
            scene
                .borrow_mut()
                .acquire_instance_mut_unchecked(self.mesh_id)
                .scale_to(
                    &(*start_scale.borrow() + &((self.target_scale - *start_scale.borrow()) * p)),
                );
        });

        out
    }
}

pub struct ScaleBy {
    pub mesh_id: usize,
    pub scale_factor: Vector3<f32>,
}

impl ScaleBy {
    pub fn new<M: MeshLike + 'static>(
        mesh_handle: MeshHandle<M>,
        scale_factor: Vector3<f32>,
    ) -> Self {
        Self {
            mesh_id: mesh_handle.id,
            scale_factor,
        }
    }
}

impl Animation<'static> for ScaleBy {
    fn into_action(
        self,
        _mesh_pool: Rc<RefCell<MeshPool>>,
        scene: Rc<RefCell<Scene>>,
    ) -> Action<'static> {
        let mut out = Action::new();
        let start_scale = Rc::new(RefCell::new(Vector3::default()));

        let scene_clone = scene.clone();
        let start_scale_clone = start_scale.clone();

        out.on_start = Box::new(move || {
            start_scale_clone.borrow_mut().clone_from(
                scene_clone
                    .borrow()
                    .acquire_instance_unchecked(self.mesh_id)
                    .scale(),
            );

            println!("{:?}", start_scale_clone.borrow());
        });
        out.on_update = Box::new(move |p, _| {
            scene
                .borrow_mut()
                .acquire_instance_mut_unchecked(self.mesh_id)
                .scale_to(&start_scale.borrow().component_mul(
                    &(Vector3::from_element(1.0)
                        + (self.scale_factor - Vector3::from_element(1.0)) * p),
                ));
        });

        out
    }
}
