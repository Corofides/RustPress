#[derive(Debug, Default)]
pub struct PostMeta {
    id: usize,
    post: usize,
    key: String,
    value: String,
}

impl PostMeta {

    pub fn generate_id() -> usize {
        0
    }

    pub fn new(post: usize, key: &str, value: &str) -> Self {
        PostMeta::default()
            .set_post(post)
            .set_key(key)
            .set_value(value)
    }

    pub fn id(&self) -> &usize {
        &self.id
    }

    pub fn set_id(mut self, id: usize) -> Self {
        self.id = id;
        self
    }

    pub fn post(&self) -> &usize {
        &self.post 
    }

    pub fn set_post(mut self, post: usize) -> Self {
        self.post = post;
        self
    }

    pub fn key(&self) -> &str {
        &self.key
    }

    pub fn set_key(mut self, key: &str) -> Self {
        self.key = key.to_string();
        self
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn set_value(mut self, value: &str) -> Self {
        self.value = value.to_string();
        self
    }

}
