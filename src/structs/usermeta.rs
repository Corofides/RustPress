use sqlx::FromRow;
use serde::Serialize;

#[derive(Serialize, FromRow, Default, Debug, Clone)]
pub struct UserMeta {
    id: u32,
    user: u32,
    key: String,
    value: String
}

impl UserMeta {

    pub fn generate_id() -> u32 {
        0
    }

    pub fn new(user: u32, key: &str, value: &str) -> Self {
        UserMeta::default()
            .set_id(Self::generate_id())
            .set_key(key)
            .set_value(value)
    }

    pub fn id(&self) -> &u32 {
        &self.id
    }

    pub fn set_id(mut self, id: u32) -> Self {
        self.id = id;
        self
    }

    pub fn user(&self) -> &u32 {
        &self.user
    }

    pub fn set_user(mut self, user: u32) -> Self {
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
