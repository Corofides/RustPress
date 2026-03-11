use crate::Post;
use crate::PostRepository;
use async_std::task;

pub struct PostService<T: PostRepository> {
    repository: T,
}

impl<T: PostRepository> PostService<T> {
    pub fn new(repository: T) -> Self {
        Self {
            repository
        }
    }
    pub fn get_posts(&self) -> Vec<Post> {
        task::block_on(async {
            self.repository
                .fetch_all()
                .await
        })
    }
}
