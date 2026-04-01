pub mod post_repository;
pub mod user_repository;
pub mod postmeta_repository;
pub mod usermeta_repository;

pub use postmeta_repository::SqlitePostmetaRepository;
pub use usermeta_repository::UserMetaRepository;

pub use user_repository::UserFilters;
pub use post_repository::PostFilters;
pub use postmeta_repository::PostmetaFilters;
pub use usermeta_repository::UserMetaFilters;

pub enum RepositoryError {
    AddItemError
}

pub trait Repository<R, F> {
    async fn add(&self, item: R) -> Result<i64, RepositoryError>;
    async fn fetch(&self, id: u32) -> Option<R>;
    async fn fetch_all(&self) -> Vec<R>;
    async fn fetch_filtered(&self, filters: F) -> Vec<R>;
    async fn exists(&self, id: u32) -> bool;
}
