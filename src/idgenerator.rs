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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_single_type() {
        let mut id_generator = IdGenerator::default();

        let first = id_generator.get_new_id(PressTypes::Post);
        assert_eq!(first, 0);

        let second = id_generator.get_new_id(PressTypes::Post);
        assert_eq!(second, 1);
    }

    #[test]
    fn check_multi_type() {
        let mut id_generator = IdGenerator::default();

        let mut posts = id_generator.get_new_id(PressTypes::Post);
        posts = id_generator.get_new_id(PressTypes::Post);
        let mut users = id_generator.get_new_id(PressTypes::User);
        posts = id_generator.get_new_id(PressTypes::Post);

        assert_eq!(posts, 2);
        assert_eq!(users, 0);
    }
}
