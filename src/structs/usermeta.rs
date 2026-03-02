#[derive(Default, Debug, Clone)]
pub struct UserMeta {
    id: usize,
    user: usize,
    key: String,
    value: String
}

impl UserMeta {

    pub fn generate_id() -> usize {
        0
    }

    pub fn new(user: usize, key: &str, value: &str) -> Self {
        UserMeta::default()
            .set_id(Self::generate_id())
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

    pub fn user(&self) -> &usize {
        &self.user
    }

    pub fn set_user(mut self, user: usize) -> Self {
        self.user = user;
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
