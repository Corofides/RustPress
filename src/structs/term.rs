
#[derive(Default, Debug, Clone)]
pub struct Term {
    id: usize,
    name: String,
    slug: String,
    group: usize,
}

impl Term {

    pub fn generate_id() -> usize {
        0
    }

    pub fn new(name: &str, slug: &str, group: usize) -> Self {
        Self::default()
            .set_id(Self::generate_id())
            .set_name(name)
            .set_slug(slug)
            .set_group(group)
    }

    pub fn id(&self) -> &usize {
        &self.id
    }

    pub fn set_id(mut self, id: usize) -> Self {
        self.id = id;
        self
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn set_name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self
    }

    pub fn slug(&self) -> &str {
        &self.slug
    }

    pub fn set_slug(mut self, slug: &str) -> Self {
        self.slug = slug.to_string();
        self
    }

    pub fn group(&self) -> &usize {
        &self.group
    }

    pub fn set_group(mut self, group: usize) -> Self {
        self.group = group;
        self
    }

}
