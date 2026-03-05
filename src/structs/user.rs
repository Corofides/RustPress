#[derive(Debug, Default)]
pub struct User {
    id: usize,
    first_name: String,
    last_name: String,
    display_name: String,
}

impl User {

    pub fn generate_id() -> usize {
        0
    }

    pub fn new(first_name: &str, last_name: &str, display_name: &str) -> Self {
        Self::default()
            .set_id(Self::generate_id())
            .set_first_name(first_name)
            .set_last_name(last_name)
            .set_display_name(display_name)
    }

    pub fn id(&self) -> &usize {
        &self.id
    }

    pub fn set_id(mut self, id: usize) -> Self {
        self.id = id;
        self
    } 

    pub fn first_name(&self) -> &str {
        &self.first_name
    }

    pub fn set_first_name(mut self, first_name: &str) -> Self {
        self.first_name = first_name.to_string();
        self
    }

    pub fn last_name(&self) -> &str {
        &self.last_name
    }

    pub fn set_last_name(mut self, last_name: &str) -> Self {
        self.last_name = last_name.to_string();
        self
    }

    pub fn display_name(&self) -> &str {
        &self.display_name
    }

    pub fn set_display_name(mut self, display_name: &str) -> Self {
        self.display_name = display_name.to_string();
        self
    }
}
