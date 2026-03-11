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
