use sqlx::FromRow;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, FromRow, Debug, Default)]
pub struct PostMeta {
    id: u32,
    post: u32,
    key: String,
    value: String,
}

impl PostMeta {

    pub fn generate_id() -> u32 {
        0
    }

    pub fn new(post: u32, key: &str, value: &str) -> Self {
        PostMeta::default()
            .set_post(post)
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

    pub fn post(&self) -> &u32 {
        &self.post 
    }

    pub fn set_post(mut self, post: u32) -> Self {
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
