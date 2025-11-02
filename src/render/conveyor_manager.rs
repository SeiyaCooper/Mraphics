use std::collections::HashMap;

use crate::render::Conveyor;

pub struct ConveyorManager<'a> {
    pub conveyor_pool: HashMap<String, Conveyor<'a>>,
}

impl<'a> ConveyorManager<'a> {
    pub fn new() -> Self {
        Self {
            conveyor_pool: HashMap::new(),
        }
    }

    pub fn acquire_attr_conveyor(&mut self, identifier: &str) -> &mut Conveyor<'a> {
        if !self.conveyor_pool.contains_key(identifier) {
            let conveyor = Conveyor::new();
            self.conveyor_pool.insert(identifier.to_string(), conveyor);
        }

        // SAFETY: Checked upon
        self.conveyor_pool.get_mut(identifier).unwrap()
    }
}
