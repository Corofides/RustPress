use crate::repository::{
    Repository,
    PostmetaFilters,
};
use crate::PostMeta;

pub struct PostMetaService<T: Repository<PostMeta, PostmetaFilters>> {
    repository: T 
}

impl<T: Repository<PostMeta, PostmetaFilters>> PostMetaService<T> {
    pub fn new(repository: T) -> Self {
        Self {
            repository
        }
    }
}
