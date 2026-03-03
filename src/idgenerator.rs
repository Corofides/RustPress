use std::collections::HashMap;

#[derive(Default)]
pub struct IdGenerator {
    types: HashMap<&str, usize>
}

impl IdGenerator {
    pub fn get_new_id(&self, item: &str) -> usize {
        let id = self.types.entry(item).or_default();
        let current_id = id.clone();
        id += 1;
        current_id
    }
}
