use crate::{
    Action, Animation, AsIntermediate, GeometryUpdater, Interpolatable, MeshHandle, MeshLike,
    MeshPool, Scene,
    anim_curve::{AnimCurve, Linear},
};
use nalgebra::{Matrix3, Vector3};
use std::{cell::RefCell, marker::PhantomData, rc::Rc};

/// Requriements to perform a [`PointwiseTransform`] or [`MatrixTransform`]
pub trait Transformable: MeshLike + AsIntermediate
where
    Self::Intermediate: Interpolatable + GeometryUpdater,
{
    /// Applies a transform function to self, and returns a intermediate representation.
    ///
    /// The intermediate representation must satisfies
    /// - [`GeometryUpdater`]: For updating geometry view.
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
    M::Intermediate: Interpolatable + GeometryUpdater,
{
    /// The unique identifier of the mesh to animate.
    pub mesh_id: usize,

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
    M::Intermediate: Interpolatable + GeometryUpdater,
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
    M::Intermediate: Interpolatable + GeometryUpdater,
{
    fn into_action(
        self,
        mesh_pool: Rc<RefCell<MeshPool>>,
        scene: Rc<RefCell<Scene>>,
    ) -> Action<'static> {
        let mut out = Action::new();
        let original = Rc::new(RefCell::new(None));
        let transformed = Rc::new(RefCell::new(None));

        let mesh_pool_clone = mesh_pool.clone();
        let oringinal_clone = original.clone();
        let transformed_clone = transformed.clone();

        out.on_start = Box::new(move || {
            *oringinal_clone.borrow_mut() = Some(
                mesh_pool_clone
                    .borrow()
                    .acquire_mesh_unchecked::<M>(self.mesh_id)
                    .as_intermediate(),
            );

            *transformed_clone.borrow_mut() = Some(
                mesh_pool_clone
                    .borrow()
                    .acquire_mesh_unchecked::<M>(self.mesh_id)
                    .apply_transform(&self.transform),
            );
        });

        out.on_update = Box::new(move |p: f32, _t: f32| {
            original
                .borrow()
                .as_ref()
                .unwrap()
                .interpolate(
                    &transformed.borrow().as_ref().unwrap(),
                    self.curve.sample(p),
                )
                .update_view(
                    &mut scene
                        .borrow_mut()
                        .acquire_instance_mut_unchecked(self.mesh_id)
                        .geometry,
                );
        });

        out
    }
}

pub struct MatrixTransform<M>
where
    M: Transformable + 'static,
    M::Intermediate: Interpolatable + GeometryUpdater,
{
    /// The unique identifier of the mesh to animate.
    pub mesh_id: usize,

    pub matrix: Matrix3<f32>,

    pub curve: Box<dyn AnimCurve>,

    _marker: PhantomData<M>,
}

impl<M> MatrixTransform<M>
where
    M: Transformable + 'static,
    M::Intermediate: Interpolatable + GeometryUpdater,
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
    M::Intermediate: Interpolatable + GeometryUpdater,
{
    fn into_action(
        self,
        mesh_pool: Rc<RefCell<MeshPool>>,
        scene: Rc<RefCell<Scene>>,
    ) -> Action<'static> {
        let mut out = Action::new();
        let original = Rc::new(RefCell::new(None));
        let transformed = Rc::new(RefCell::new(None));

        let mesh_pool_clone = mesh_pool.clone();
        let oringinal_clone = original.clone();
        let transformed_clone = transformed.clone();

        out.on_start = Box::new(move || {
            *oringinal_clone.borrow_mut() = Some(
                mesh_pool_clone
                    .borrow()
                    .acquire_mesh_unchecked::<M>(self.mesh_id)
                    .as_intermediate(),
            );

            *transformed_clone.borrow_mut() = Some(
                mesh_pool_clone
                    .borrow()
                    .acquire_mesh_unchecked::<M>(self.mesh_id)
                    .apply_transform(|point: &[f32; 3]| {
                        let transformed = self.matrix * &Vector3::from_row_slice(point);
                        return [transformed[0], transformed[1], transformed[2]];
                    }),
            );
        });

        out.on_update = Box::new(move |p: f32, _t: f32| {
            original
                .borrow()
                .as_ref()
                .unwrap()
                .interpolate(
                    &transformed.borrow().as_ref().unwrap(),
                    self.curve.sample(p),
                )
                .update_view(
                    &mut scene
                        .borrow_mut()
                        .acquire_instance_mut_unchecked(self.mesh_id)
                        .geometry,
                );
        });

        out
    }
}
