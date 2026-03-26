use crate::repository::{
    Repository,
    PostmetaFilters,
};
use crate::PostMeta;
use async_std::task;

pub struct PostMetaService<T: Repository<PostMeta, PostmetaFilters>> {
    repository: T 
}

impl<T: Repository<PostMeta, PostmetaFilters>> PostMetaService<T> {
    pub fn new(repository: T) -> Self {
        Self {
            repository
        }
    }
    pub fn get_post_meta(&self) -> Vec<PostMeta> {
        task::block_on(async {
            self.repository
                .fetch_all()
                .await
        })
    }
    pub fn get_post_metum(&self, id: u32) -> Option<PostMeta> {
        task::block_on(async {
            self.repository
                .fetch(id)
                .await
        })
    }
}
