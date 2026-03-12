pub mod post_repository;
pub mod user_repository;

pub enum RepositoryError {
    AddItemError
}

pub trait Repository<R, F> {
    async fn add(&self, item: R) -> Result<(), RepositoryError>;
    async fn fetch(&self, id: u32) -> Option<R>;
    async fn fetch_all(&self) -> Vec<R>;
    async fn fetch_filtered(&self, filters: F) -> Vec<R>;
}
