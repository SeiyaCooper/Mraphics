use std::{collections::HashMap, hash::Hash};

use crate::Conveyor;

pub struct ConveyorManager<Key: Hash + Eq + ToOwned<Owned = Key>> {
    pub conveyor_pool: HashMap<Key, Conveyor>,
}

impl<Key: Hash + Eq + ToOwned<Owned = Key>> ConveyorManager<Key> {
    pub fn new() -> Self {
        Self {
            conveyor_pool: HashMap::new(),
        }
    }

    pub fn acquire_conveyor(&mut self, identifier: &Key) -> &mut Conveyor {
        if !self.conveyor_pool.contains_key(identifier) {
            let conveyor = Conveyor::new();
            self.conveyor_pool.insert(identifier.to_owned(), conveyor);
        }

        // SAFETY: Checked upon
        self.conveyor_pool.get_mut(identifier).unwrap()
    }
}
