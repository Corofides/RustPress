use crate::Post;
use crate::PostRepository;
use crate::repository::RepositoryError;
use async_std::task;
use crate::repository::{
    post_repository::PostFilters,
    Repository,
};

pub struct PostService<T: Repository<Post, PostFilters>> {
    repository: T,
}

impl<T: Repository<Post, PostFilters>> PostService<T> {
    pub fn new(repository: T) -> Self {
        Self {
            repository
        }
    }
    pub fn get_post(&self, id: u32) -> Option<Post> {
        task::block_on(async {
            self.repository
                .fetch(id)
                .await
        })
        //async fn fetch(&self, id: u32) -> Option<Post> {

    }
    pub fn get_posts(&self) -> Vec<Post> {
        task::block_on(async {
            self.repository
                .fetch_all()
                .await
        })
    }
    pub fn create_post(&self, post: Post) -> Result<(), RepositoryError> {
        task::block_on(async {
            self.repository
                .add(post)
                .await
        })
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use sqlx::Error;

    struct MockRepo;

    impl PostRepository for MockRepo {
        fn fetch_all(&self) -> impl Future<Output = Vec<Post>> {
            async {
                vec![]
            }
        }
        fn create_post(&self, post: &Post) -> impl Future<Output = Result<(), Error>> {
            async {
                Ok(())
            }
        }
    }

    fn test_get_posts() {
        let mock = MockRepo;
        let post_service = PostService::new(mock);

        let posts = post_service.get_posts();

        assert!(false);
    }
}
