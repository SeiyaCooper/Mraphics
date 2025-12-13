use crate::{RenderInstance, Renderable};
use std::collections::HashMap;

pub struct Scene {
    pub instances: Vec<RenderInstance>,
    instance_map: HashMap<usize, usize>,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            instances: Vec::new(),
            instance_map: HashMap::new(),
        }
    }

    pub fn add_renderable<R: Renderable>(&mut self, renderable: &R) {
        self.instances.push(renderable.build_instance());
        self.instance_map
            .insert(renderable.identifier(), self.instances.len() - 1);
    }

    pub fn remove_renderable(&mut self, identifier: &usize) -> Option<RenderInstance> {
        let remove_index = *self.instance_map.get(identifier)?;
        let swap_index = self.instances.len() - 1;

        for index in self.instance_map.values_mut() {
            if *index == swap_index {
                *index = remove_index;
            }
        }

        Some(self.instances.swap_remove(remove_index))
    }

    pub fn acquire_instance(&self, identifier: usize) -> Option<&RenderInstance> {
        let index = self.instance_map.get(&identifier)?;
        self.instances.get(*index)
    }

    pub fn acquire_instance_unchecked(&self, identifier: usize) -> &RenderInstance {
        &self.instances[*self.instance_map.get(&identifier).unwrap()]
    }

    pub fn acquire_instance_mut(&mut self, identifier: usize) -> Option<&mut RenderInstance> {
        let index = self.instance_map.get(&identifier)?;
        self.instances.get_mut(*index)
    }

    pub fn acquire_instance_mut_unchecked(&mut self, identifier: usize) -> &mut RenderInstance {
        &mut self.instances[*self.instance_map.get(&identifier).unwrap()]
    }
}
