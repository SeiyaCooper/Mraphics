use std::collections::HashMap;

use crate::render::Conveyor;

pub struct ConveyorStatus<'reference, 'conveyor> {
    pub reference: &'reference mut Conveyor<'conveyor>,
    pub is_new: bool,
}

pub struct ConveyorManager<'a> {
    pub conveyor_pool: HashMap<String, Conveyor<'a>>,
}

impl<'a> ConveyorManager<'a> {
    pub fn new() -> Self {
        Self {
            conveyor_pool: HashMap::new(),
        }
    }

    pub fn acquire_attr_conveyor(&mut self, identifier: &str) -> ConveyorStatus<'_, 'a> {
        if !self.conveyor_pool.contains_key(identifier) {
            let conveyor = Conveyor::new();
            self.conveyor_pool.insert(identifier.to_string(), conveyor);

            return ConveyorStatus {
                reference: self.conveyor_pool.get_mut(identifier).unwrap(),
                is_new: true,
            };
        }

        // SAFETY: Checked upon
        ConveyorStatus {
            reference: self.conveyor_pool.get_mut(identifier).unwrap(),
            is_new: false,
        }
    }
}
