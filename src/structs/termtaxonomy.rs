
#[derive(Debug, Clone, Default)]
pub struct TermTaxonomy {
    id: usize,
    term_id: usize,
    taxonomy: String,
    description: String,
    parent: usize,
}

impl TermTaxonomy {

    pub fn new(id: usize, term_id: usize, taxonomy: &str, description: &str, parent: usize) -> Self {
        Self::default()
            .set_id(id)
            .set_term_id(term_id)
            .set_taxonomy(taxonomy)
            .set_description(description)
            .set_parent(parent)
    }

    pub fn id(&self) -> &usize {
        &self.id
    }

    pub fn set_id(mut self, id: usize) -> Self {
        self.id = id;
        self
    }

    pub fn term_id(&self) -> &usize {
        &self.term_id
    }

    pub fn set_term_id(mut self, term_id: usize) -> Self {
        self.term_id = term_id;
        self
    }

    pub fn taxonomy(&self) -> &str {
        &self.taxonomy
    }

    pub fn set_taxonomy(mut self, taxonomy: &str) -> Self {
        self.taxonomy = taxonomy.to_string();
        self
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn set_description(mut self, description: &str) -> Self {
        self.description = description.to_string();
        self
    }

    pub fn parent(&self) -> &usize {
        &self.parent
    }

    pub fn set_parent(mut self, parent: usize) -> Self {
        self.parent = parent;
        self
    }

}
