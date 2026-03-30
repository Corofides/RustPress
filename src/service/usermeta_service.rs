use crate::UserMeta;
use crate::repository::{
    UserMetaFilters,
    Repository,
};
use async_std::task;
use crate::repository::RepositoryError;

pub struct UserMetaService<T: Repository<UserMeta, UserMetaFilters>> {
    repository: T
}

impl<T: Repository<UserMeta, UserMetaFilters>> UserMetaService<T> {
    pub fn new(repository: T) -> Self {
        Self {
            repository
        }
    }
    pub fn get_user_meta(&self) -> Vec<UserMeta> {
        task::block_on(async {
            self.repository
                .fetch_all()
                .await
        })
    }
    pub fn get_user_metum(&self, id: u32) -> Option<UserMeta> {
        task::block_on(async {
            self.repository
                .fetch(id)
                .await
        })
    }
    pub fn add_user_meta(&self, user_meta: UserMeta) -> Result<i64, RepositoryError> {
        task::block_on(async {
            self.repository
                .add(user_meta)
                .await
        })
    }
}
