use crate::{
    Action, Animation, MeshHandle, MeshLike, MeshPool, MraphicsID, RenderInstance, Scene,
    anim_curve::{AnimCurve, EaseInOutCubic},
};
use nalgebra::{UnitQuaternion, UnitVector3, Vector3};
use std::{cell::RefCell, marker::PhantomData, rc::Rc};

pub struct MeshAnimation<'res, M, Update, Start, Stop, C>
where
    M: MeshLike + 'static,
    Update: FnMut(&mut M, &mut RenderInstance, f32, f32) + 'res,
    Start: FnMut() + 'res,
    Stop: FnMut() + 'res,
    C: AnimCurve + 'static,
{
    pub mesh_id: MraphicsID,
    pub on_update: Update,
    pub on_start: Start,
    pub on_stop: Stop,
    pub curve: C,

    _marker: PhantomData<&'res M>,
}

impl<'res, M: MeshLike + 'static>
    MeshAnimation<'res, M, fn(&mut M, &mut RenderInstance, f32, f32), fn(), fn(), EaseInOutCubic>
{
    pub fn new(mesh_handle: &MeshHandle<M>) -> Self {
        Self {
            mesh_id: mesh_handle.id,
            on_update: |_, _, _, _| {},
            on_start: || {},
            on_stop: || {},
            curve: EaseInOutCubic,
            _marker: PhantomData,
        }
    }
}

impl<'res, M, Update, Start, Stop, C> MeshAnimation<'res, M, Update, Start, Stop, C>
where
    M: MeshLike + 'static,
    Update: FnMut(&mut M, &mut RenderInstance, f32, f32) + 'res,
    Start: FnMut() + 'res,
    Stop: FnMut() + 'res,
    C: AnimCurve + 'static,
{
    pub fn with_on_update<F: FnMut(&mut M, &mut RenderInstance, f32, f32) + 'res>(
        self,
        closure: F,
    ) -> MeshAnimation<'res, M, F, Start, Stop, C> {
        MeshAnimation {
            mesh_id: self.mesh_id,
            on_update: closure,
            on_start: self.on_start,
            on_stop: self.on_stop,
            curve: self.curve,
            _marker: PhantomData,
        }
    }

    pub fn with_on_start<F: FnMut() + 'res>(
        self,
        closure: F,
    ) -> MeshAnimation<'res, M, Update, F, Stop, C> {
        MeshAnimation {
            mesh_id: self.mesh_id,
            on_update: self.on_update,
            on_start: closure,
            on_stop: self.on_stop,
            curve: self.curve,
            _marker: PhantomData,
        }
    }

    pub fn with_on_stop<F: FnMut() + 'res>(
        self,
        closure: F,
    ) -> MeshAnimation<'res, M, Update, Start, F, C> {
        MeshAnimation {
            mesh_id: self.mesh_id,
            on_update: self.on_update,
            on_start: self.on_start,
            on_stop: closure,
            curve: self.curve,
            _marker: PhantomData,
        }
    }

    pub fn with_curve<T: AnimCurve + 'static>(
        self,
        curve: T,
    ) -> MeshAnimation<'res, M, Update, Start, Stop, T> {
        MeshAnimation {
            mesh_id: self.mesh_id,
            on_update: self.on_update,
            on_start: self.on_start,
            on_stop: self.on_stop,
            curve,
            _marker: PhantomData,
        }
    }
}

impl<'res, M, Update, Start, Stop, C> Animation<'res>
    for MeshAnimation<'res, M, Update, Start, Stop, C>
where
    M: MeshLike + 'static,
    Update: FnMut(&mut M, &mut RenderInstance, f32, f32) + 'res,
    Start: FnMut() + 'res,
    Stop: FnMut() + 'res,
    C: AnimCurve + 'static,
{
    fn into_action(
        mut self,
        mesh_pool: Rc<RefCell<MeshPool>>,
        scene: Rc<RefCell<Scene>>,
    ) -> Action<'res> {
        let mut out = Action::new();

        out.on_start = Box::new(self.on_start);
        out.on_stop = Box::new(self.on_stop);

        out.on_update = Box::new(move |progress, elapsed_time| {
            (self.on_update)(
                mesh_pool
                    .borrow_mut()
                    .acquire_mesh_mut_unchecked(self.mesh_id),
                scene
                    .borrow_mut()
                    .acquire_instance_mut_unchecked(self.mesh_id),
                self.curve.sample(progress),
                elapsed_time,
            )
        });

        out
    }
}

/// Rotates the mesh around a given axis by a given angle.
pub struct RotateAxisAngle {
    /// The unique identifier of the mesh to animate.
    pub mesh_id: MraphicsID,

    /// The axis of rotation, normalized to unit length.
    pub axis: UnitVector3<f32>,

    /// The rotation angle in radians for this animation.
    pub angle_rad: f32,

    pub curve: Box<dyn AnimCurve>,
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
            curve: Box::new(EaseInOutCubic),
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
            curve: Box::new(EaseInOutCubic),
        }
    }

    pub fn with_curve<Curve: AnimCurve + 'static>(mut self, curve: Curve) -> Self {
        self.curve = Box::new(curve);
        self
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
                    &(UnitQuaternion::from_axis_angle(
                        &self.axis,
                        self.angle_rad * self.curve.sample(p),
                    ) * &*start_rotation.borrow()),
                );
        });

        out
    }
}

/// Shifts the mesh to the specific place
pub struct MoveTo {
    pub mesh_id: MraphicsID,
    pub target_place: Vector3<f32>,

    pub curve: Box<dyn AnimCurve>,
}

impl MoveTo {
    pub fn new<M: MeshLike + 'static>(
        mesh_handle: &MeshHandle<M>,
        target_place: Vector3<f32>,
    ) -> Self {
        Self {
            mesh_id: mesh_handle.id,
            target_place,
            curve: Box::new(EaseInOutCubic),
        }
    }

    pub fn with_curve<Curve: AnimCurve + 'static>(mut self, curve: Curve) -> Self {
        self.curve = Box::new(curve);
        self
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
                    &(*start_place.borrow()
                        + &((self.target_place - *start_place.borrow()) * self.curve.sample(p))),
                );
        });

        out
    }
}

pub struct ScaleTo {
    pub mesh_id: MraphicsID,
    pub target_scale: Vector3<f32>,

    pub curve: Box<dyn AnimCurve>,
}

impl ScaleTo {
    pub fn new<M: MeshLike + 'static>(
        mesh_handle: MeshHandle<M>,
        target_scale: Vector3<f32>,
    ) -> Self {
        Self {
            mesh_id: mesh_handle.id,
            target_scale,
            curve: Box::new(EaseInOutCubic),
        }
    }

    pub fn with_curve<Curve: AnimCurve + 'static>(mut self, curve: Curve) -> Self {
        self.curve = Box::new(curve);
        self
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
                    &(*start_scale.borrow()
                        + &((self.target_scale - *start_scale.borrow()) * self.curve.sample(p))),
                );
        });

        out
    }
}

pub struct ScaleBy {
    pub mesh_id: MraphicsID,
    pub scale_factor: Vector3<f32>,

    pub curve: Box<dyn AnimCurve>,
}

impl ScaleBy {
    pub fn new<M: MeshLike + 'static>(
        mesh_handle: MeshHandle<M>,
        scale_factor: Vector3<f32>,
    ) -> Self {
        Self {
            mesh_id: mesh_handle.id,
            scale_factor,

            curve: Box::new(EaseInOutCubic),
        }
    }

    pub fn with_curve<Curve: AnimCurve + 'static>(mut self, curve: Curve) -> Self {
        self.curve = Box::new(curve);
        self
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
        });
        out.on_update = Box::new(move |p, _| {
            scene
                .borrow_mut()
                .acquire_instance_mut_unchecked(self.mesh_id)
                .scale_to(&start_scale.borrow().component_mul(
                    &(Vector3::from_element(1.0)
                        + (self.scale_factor - Vector3::from_element(1.0)) * self.curve.sample(p)),
                ));
        });

        out
    }
}
