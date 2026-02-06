use std::{any::TypeId, collections::HashMap, mem, ptr};

use crate::{MeshHandle, MeshLike};

/// A type-erased container for storing any type that implements `MeshLike`.
///
/// Similar to [`Box<dyn Any>`].
pub struct MeshBox {
    data: *mut u8,

    type_id: TypeId,

    size: usize,
    align: usize,
    drop_fn: unsafe fn(*mut u8),

    update_fn: unsafe fn(*mut u8),
}

impl MeshBox {
    pub fn new<M: MeshLike + 'static>(mesh: M) -> Self {
        let layout = std::alloc::Layout::new::<M>();
        let ptr = unsafe { std::alloc::alloc(layout) };

        unsafe {
            ptr::write(ptr as *mut M, mesh);
        }

        Self {
            data: ptr,

            type_id: TypeId::of::<M>(),

            size: mem::size_of::<M>(),
            align: mem::align_of::<M>(),
            drop_fn: |ptr| unsafe {
                let layout = std::alloc::Layout::new::<M>();
                ptr::drop_in_place(ptr as *mut M);
                std::alloc::dealloc(ptr, layout);
            },

            update_fn: |ptr| unsafe {
                (&mut *(ptr as *mut M)).update();
            },
        }
    }

    /// Returns the size in bytes of the stored mesh type.
    pub fn size(&self) -> usize {
        self.size
    }

    /// Returns the alignment requirement of the stored mesh type.
    pub fn align(&self) -> usize {
        self.align
    }

    /// Checks if the stored mesh is of given type `M`.
    pub fn is<M: MeshLike + 'static>(&self) -> bool {
        self.type_id == TypeId::of::<M>()
    }

    /// Attempts to downcast the stored mesh to a reference of given type `M`.
    /// Returns `Some(&M)` if the types match, `None` otherwise.
    pub fn downcast_ref<M: MeshLike + 'static>(&self) -> Option<&M> {
        if self.is::<M>() {
            Some(self.downcast_ref_unchecked::<M>())
        } else {
            None
        }
    }

    /// Downcasts the stored mesh to a reference of type `M` without type checking.
    ///
    /// # Safety
    /// Caller must ensure the stored mesh is actually of type `M`.
    /// Incorrect usage leads to undefined behavior.
    pub fn downcast_ref_unchecked<M: MeshLike + 'static>(&self) -> &M {
        unsafe { &*(self.data as *const M) }
    }

    /// Attempts to downcast the stored mesh to a mutable reference of given type `M`.
    /// Returns `Some(&mut M)` if the types match, `None` otherwise.
    pub fn downcast_mut<M: MeshLike + 'static>(&mut self) -> Option<&mut M> {
        if self.is::<M>() {
            Some(self.downcast_mut_unchecked::<M>())
        } else {
            None
        }
    }

    /// Downcasts the stored mesh to a mutable reference of type `M` without type checking.
    ///
    /// # Safety
    /// Caller must ensure the stored mesh is actually of type `M`.
    /// Incorrect usage leads to undefined behavior.
    pub fn downcast_mut_unchecked<M: MeshLike + 'static>(&mut self) -> &mut M {
        unsafe { &mut *(self.data as *mut M) }
    }

    /// Triggers [`MeshLike::update`] for the stored mesh.
    pub fn update_mesh(&mut self) {
        // SAFETY: `self.update_fn` and `self.data` are initialized with the same type in [`MeshBox::new`],
        // and the only way to build a [`MeshBox`] is by calling [`MeshBox::new`]
        unsafe { (self.update_fn)(self.data) }
    }
}

impl Drop for MeshBox {
    fn drop(&mut self) {
        unsafe { (self.drop_fn)(self.data) }
    }
}

pub struct MeshPool {
    pub meshes: Vec<MeshBox>,
    mesh_map: HashMap<usize, usize>,
}

impl MeshPool {
    pub fn new() -> Self {
        Self {
            meshes: Vec::new(),
            mesh_map: HashMap::new(),
        }
    }

    pub fn add_mesh<M: MeshLike + 'static>(&mut self, mesh: M) -> MeshHandle<M> {
        let id = mesh.identifier();

        self.meshes.push(MeshBox::new(mesh));
        self.mesh_map.insert(id, self.meshes.len() - 1);

        MeshHandle::<M>::new(id)
    }

    /// Triggers [`MeshLike::update`] for the mesh specified with id.
    pub fn update_mesh(&mut self, id: usize) {
        self.meshes[*self.mesh_map.get(&id).unwrap()].update_mesh();
    }

    pub fn acquire_mesh<M: MeshLike + 'static>(&self, id: usize) -> Option<&M> {
        self.meshes[*self.mesh_map.get(&id).unwrap()].downcast_ref::<M>()
    }

    pub fn acquire_mesh_unchecked<M: MeshLike + 'static>(&self, id: usize) -> &M {
        self.meshes[*self.mesh_map.get(&id).unwrap()]
            .downcast_ref::<M>()
            .unwrap()
    }

    pub fn acquire_mesh_mut<M: MeshLike + 'static>(&mut self, id: usize) -> Option<&mut M> {
        self.meshes[*self.mesh_map.get(&id).unwrap()].downcast_mut::<M>()
    }

    pub fn acquire_mesh_mut_unchecked<M: MeshLike + 'static>(&mut self, id: usize) -> &mut M {
        self.meshes[*self.mesh_map.get(&id).unwrap()]
            .downcast_mut::<M>()
            .unwrap()
    }
}
