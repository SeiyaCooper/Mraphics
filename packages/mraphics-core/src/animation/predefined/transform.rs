use crate::{
    Action, Animation, InstanceUpdater, Interpolatable, MeshHandle, MeshLike, MeshPool, MraphicsID,
    Representable, Scene,
    anim_curve::{AnimCurve, Linear},
};
use nalgebra::{Matrix3, Vector3};
use std::{cell::RefCell, marker::PhantomData, rc::Rc};

/// Requriements to perform a [`PointwiseTransform`] or [`MatrixTransform`]
pub trait Transformable: MeshLike + Representable
where
    Self::Intermediate: Interpolatable + InstanceUpdater,
{
    /// Applies a transform function to self, and returns a intermediate representation.
    ///
    /// The intermediate representation must satisfies
    /// - [`InstanceUpdater`]: For updating geometry view.
    /// - [`Interpolatable`]: For performing a tweening animation.
    fn apply_transform<Trans: Fn(&[f32; 3]) -> [f32; 3]>(
        &self,
        transform: Trans,
    ) -> Self::Intermediate;
}

pub struct PointwiseTransform<Trans, M>
where
    Trans: Fn(&[f32; 3]) -> [f32; 3] + 'static,
    M: Transformable + 'static,
    M::Intermediate: Interpolatable + InstanceUpdater,
{
    /// The unique identifier of the mesh to animate.
    pub mesh_id: MraphicsID,

    /// The transform function to apply.
    pub transform: Trans,

    /// Animation curve.
    pub curve: Box<dyn AnimCurve>,

    _marker: PhantomData<M>,
}

impl<Trans, M> PointwiseTransform<Trans, M>
where
    Trans: Fn(&[f32; 3]) -> [f32; 3] + 'static,
    M: Transformable + 'static,
    M::Intermediate: Interpolatable + InstanceUpdater,
{
    pub fn new(mesh_handle: &MeshHandle<M>, trans: Trans) -> Self {
        Self {
            mesh_id: mesh_handle.id,
            transform: trans,

            curve: Box::new(Linear),

            _marker: PhantomData,
        }
    }

    pub fn with_curve<Curve: AnimCurve + 'static>(mut self, curve: Curve) -> Self {
        self.curve = Box::new(curve);
        self
    }
}

impl<Trans, M> Animation<'static> for PointwiseTransform<Trans, M>
where
    Trans: Fn(&[f32; 3]) -> [f32; 3] + 'static,
    M: Transformable + 'static,
    M::Intermediate: Interpolatable + InstanceUpdater,
{
    fn into_action(
        self,
        mesh_pool: Rc<RefCell<MeshPool>>,
        scene: Rc<RefCell<Scene>>,
    ) -> Action<'static> {
        let mut out = Action::new();
        let original = Rc::new(RefCell::new(None));
        let transformed = Rc::new(RefCell::new(None));

        out.on_start = Box::new({
            let oringinal = original.clone();
            let transformed = transformed.clone();
            let mesh_pool = mesh_pool.clone();
            move || {
                *oringinal.borrow_mut() = Some(
                    mesh_pool
                        .borrow()
                        .acquire_mesh_unchecked::<M>(self.mesh_id)
                        .as_intermediate(),
                );

                *transformed.borrow_mut() = Some(
                    mesh_pool
                        .borrow()
                        .acquire_mesh_unchecked::<M>(self.mesh_id)
                        .apply_transform(&self.transform),
                );
            }
        });

        out.on_update = Box::new({
            let original = original.clone();
            let transformed = transformed.clone();
            move |p: f32, _t: f32| {
                original
                    .borrow()
                    .as_ref()
                    .unwrap()
                    .interpolate(
                        &transformed.borrow().as_ref().unwrap(),
                        self.curve.sample(p),
                    )
                    .update_instance(
                        &mut scene
                            .borrow_mut()
                            .acquire_instance_mut_unchecked(self.mesh_id),
                    );
            }
        });

        out.on_stop = Box::new({
            let transformed = transformed.clone();
            move || {
                mesh_pool
                    .borrow_mut()
                    .acquire_mesh_mut_unchecked::<M>(self.mesh_id)
                    .update_from_intermediate(transformed.borrow().as_ref().unwrap());
            }
        });

        out
    }
}

pub struct MatrixTransform<M>
where
    M: Transformable + 'static,
    M::Intermediate: Interpolatable + InstanceUpdater,
{
    /// The unique identifier of the mesh to animate.
    pub mesh_id: MraphicsID,

    pub matrix: Matrix3<f32>,

    pub curve: Box<dyn AnimCurve>,

    _marker: PhantomData<M>,
}

impl<M> MatrixTransform<M>
where
    M: Transformable + 'static,
    M::Intermediate: Interpolatable + InstanceUpdater,
{
    pub fn new(mesh_handle: &MeshHandle<M>, matrix: Matrix3<f32>) -> Self {
        Self {
            mesh_id: mesh_handle.id,
            matrix,

            curve: Box::new(Linear),

            _marker: PhantomData,
        }
    }

    pub fn with_curve<Curve: AnimCurve + 'static>(mut self, curve: Curve) -> Self {
        self.curve = Box::new(curve);
        self
    }
}

impl<M> Animation<'static> for MatrixTransform<M>
where
    M: Transformable + 'static,
    M::Intermediate: Interpolatable + InstanceUpdater,
{
    fn into_action(
        self,
        mesh_pool: Rc<RefCell<MeshPool>>,
        scene: Rc<RefCell<Scene>>,
    ) -> Action<'static> {
        let mut out = Action::new();
        let original = Rc::new(RefCell::new(None));
        let transformed = Rc::new(RefCell::new(None));

        out.on_start = Box::new({
            let oringinal = original.clone();
            let transformed = transformed.clone();
            let mesh_pool = mesh_pool.clone();
            move || {
                *oringinal.borrow_mut() = Some(
                    mesh_pool
                        .borrow()
                        .acquire_mesh_unchecked::<M>(self.mesh_id)
                        .as_intermediate(),
                );

                *transformed.borrow_mut() = Some(
                    mesh_pool
                        .borrow()
                        .acquire_mesh_unchecked::<M>(self.mesh_id)
                        .apply_transform(|point: &[f32; 3]| {
                            let transformed = self.matrix * &Vector3::from_row_slice(point);
                            return [transformed[0], transformed[1], transformed[2]];
                        }),
                );
            }
        });

        out.on_update = Box::new({
            let original = original.clone();
            let transformed = transformed.clone();
            move |p: f32, _t: f32| {
                original
                    .borrow()
                    .as_ref()
                    .unwrap()
                    .interpolate(
                        &transformed.borrow().as_ref().unwrap(),
                        self.curve.sample(p),
                    )
                    .update_instance(
                        &mut scene
                            .borrow_mut()
                            .acquire_instance_mut_unchecked(self.mesh_id),
                    );
            }
        });

        out.on_stop = Box::new({
            let transformed = transformed.clone();
            move || {
                mesh_pool
                    .borrow_mut()
                    .acquire_mesh_mut_unchecked::<M>(self.mesh_id)
                    .update_from_intermediate(transformed.borrow().as_ref().unwrap());
            }
        });

        out
    }
}
