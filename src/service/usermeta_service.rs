use crate::UserMeta;
use crate::repository::{
    UserMetaFilters,
    Repository,
};

pub struct UserMetaService<T: Repository<UserMeta, UserMetaFilters>> {
    repository: T
}

impl<T: Repository<UserMeta, UserMetaFilters>> UserMetaService<T> {
    pub fn new(repository: T) -> Self {
        Self {
            repository
        }
    }
}
