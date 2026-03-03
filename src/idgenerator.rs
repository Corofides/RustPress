use std::collections::HashMap;
use crate::structs::presstypes::PressTypes;

#[derive(Default)]
pub struct IdGenerator {
    types: HashMap<PressTypes, usize>
}

impl IdGenerator {
    pub fn get_new_id(&mut self, item: PressTypes) -> usize {
        let id = self.types.entry(item).or_default();
        let current_id = id.clone();
        *id += 1;
        current_id
    }
}
