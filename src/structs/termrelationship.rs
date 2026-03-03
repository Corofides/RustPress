
#[derive(Debug, Default, Clone)]
pub struct TermRelationship {
    object_id: usize,
    term_taxonomy_id: usize,
    term_order: usize,
}

impl TermRelationship {

    pub fn new(object_id: usize, term_taxonomy_id: usize, term_order: usize) -> Self {
        Self::default()
            .set_object_id(object_id)
            .set_term_taxonomy_id(term_taxonomy_id)
            .set_term_order(term_order)
    }

    pub fn object_id(&self) -> &usize {
        &self.object_id
    }

    pub fn set_object_id(mut self, object_id: usize) -> Self {
        self.object_id = object_id;
        self
    }

    pub fn term_taxonomy_id(&self) -> &usize {
        &self.term_taxonomy_id
    }

    pub fn set_term_taxonomy_id(mut self, term_taxonomy_id: usize) -> Self {
        self.term_taxonomy_id = term_taxonomy_id;
        self
    }

    pub fn term_order(&self) -> &usize {
        &self.term_order
    }

    pub fn set_term_order(mut self, term_order: usize) -> Self {
        self.term_order = term_order;
        self
    }
}
