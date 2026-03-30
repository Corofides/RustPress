use crate::repository::{
    Repository,
    PostmetaFilters,
};
use crate::PostMeta;
use crate::repository::RepositoryError;
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

    pub fn add_post_metum(&self, post_meta: PostMeta) -> Result<i64, RepositoryError> {
        task::block_on(async {
            self.repository
                .add(post_meta)
                .await
        })
    }
}
