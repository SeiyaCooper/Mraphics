use nalgebra::{Matrix3, Vector3};

use crate::{Action, Animation, MeshHandle, MeshLike, MeshPool, Scene};
use std::{cell::RefCell, marker::PhantomData, rc::Rc};

pub trait Transformable: MeshLike {
    fn apply_transform<Trans: Fn(&[f32; 3]) -> [f32; 3]>(
        &self,
        transform: Trans,
        progress: f32,
    ) -> Self;
}

pub struct PointwiseTransform<
    Trans: Fn(&[f32; 3]) -> [f32; 3] + 'static,
    M: Transformable + 'static,
> {
    /// The unique identifier of the mesh to animate.
    pub mesh_id: usize,

    pub transform: Trans,

    _marker: PhantomData<M>,
}

impl<Trans: Fn(&[f32; 3]) -> [f32; 3] + 'static, M: Transformable + 'static>
    PointwiseTransform<Trans, M>
{
    pub fn new(mesh_handle: &MeshHandle<M>, trans: Trans) -> Self {
        Self {
            mesh_id: mesh_handle.id,
            transform: trans,
            _marker: PhantomData,
        }
    }
}

impl<Trans: Fn(&[f32; 3]) -> [f32; 3] + 'static, M: Transformable + 'static> Animation<'static>
    for PointwiseTransform<Trans, M>
{
    fn into_action(
        self,
        mesh_pool: Rc<RefCell<MeshPool>>,
        scene: Rc<RefCell<Scene>>,
    ) -> Action<'static> {
        let mut out = Action::new();

        out.on_update = Box::new(move |p: f32, _t: f32| {
            let transformed = mesh_pool
                .borrow_mut()
                .acquire_mesh_mut_unchecked::<M>(self.mesh_id)
                .apply_transform(&self.transform, p);

            transformed.update_geometry_view(
                &mut scene
                    .borrow_mut()
                    .acquire_instance_mut_unchecked(self.mesh_id)
                    .geometry,
            );
        });

        out
    }
}

pub struct MatrixTransform<M: Transformable + 'static> {
    /// The unique identifier of the mesh to animate.
    pub mesh_id: usize,

    matrix: Matrix3<f32>,

    _marker: PhantomData<M>,
}

impl<M: Transformable + 'static> MatrixTransform<M> {
    pub fn new(mesh_handle: &MeshHandle<M>, matrix: Matrix3<f32>) -> Self {
        Self {
            mesh_id: mesh_handle.id,
            matrix,
            _marker: PhantomData,
        }
    }
}

impl<M: Transformable + 'static> Animation<'static> for MatrixTransform<M> {
    fn into_action(
        self,
        mesh_pool: Rc<RefCell<MeshPool>>,
        scene: Rc<RefCell<Scene>>,
    ) -> Action<'static> {
        let mut out = Action::new();

        out.on_update = Box::new(move |p: f32, _t: f32| {
            let transformed = mesh_pool
                .borrow_mut()
                .acquire_mesh_mut_unchecked::<M>(self.mesh_id)
                .apply_transform(
                    |point: &[f32; 3]| {
                        let transformed = self.matrix * &Vector3::from_row_slice(point);
                        return [transformed[0], transformed[1], transformed[2]];
                    },
                    p,
                );

            transformed.update_geometry_view(
                &mut scene
                    .borrow_mut()
                    .acquire_instance_mut_unchecked(self.mesh_id)
                    .geometry,
            );
        });

        out
    }
}
