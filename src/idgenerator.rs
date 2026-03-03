use std::collections::HashMap;
use crate::structs::presstype::PressType;

#[derive(Default)]
pub struct IdGenerator {
    types: HashMap<PressType, usize>
}

impl IdGenerator {
    pub fn get_new_id(&mut self, item: PressType) -> usize {
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

        let first = id_generator.get_new_id(PressType::Post);
        assert_eq!(first, 0);

        let second = id_generator.get_new_id(PressType::Post);
        assert_eq!(second, 1);
    }

    #[test]
    fn check_multi_type() {
        let mut id_generator = IdGenerator::default();

        let mut posts = id_generator.get_new_id(PressType::Post);
        posts = id_generator.get_new_id(PressType::Post);
        let mut users = id_generator.get_new_id(PressType::User);
        posts = id_generator.get_new_id(PressType::Post);

        assert_eq!(posts, 2);
        assert_eq!(users, 0);
    }
}
