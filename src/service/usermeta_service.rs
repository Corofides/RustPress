use crate::UserMeta;
use crate::repository::{
    UserMetaFilters,
    Repository,
};
use async_std::task;

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
}
