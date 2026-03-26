
use crate::structs::post::Post;
use std::sync::{
    Arc,
    Mutex,
};
use sqlx::{
    QueryBuilder,
    Sqlite,
    SqlitePool,
    Error,
};
use super::{
    Repository,
    RepositoryError,
};

pub struct PostFilters {
}

pub trait PostRepository {
    fn fetch_all(&self) -> impl Future<Output = Vec<Post>>;
    fn create_post(&self, post: &Post) -> impl Future<Output = Result<(), Error>>;
}

pub struct SqlitePostRepository {
    pool: SqlitePool,
}

impl SqlitePostRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self {
            pool
        }
    }
}

impl Repository<Post, PostFilters> for SqlitePostRepository {
    async fn add(&self, post: Post) -> Result<(), RepositoryError> {
        sqlx::query("INSERT INTO posts (
                    title, content, author
                ) VALUES (
                    ?,
                    ?,
                    ?
                )
            ")
            .bind(post.title())
            .bind(post.content())
            .bind(post.author())
            .execute(&self.pool)
            .await;

        Ok(())
    }
    async fn fetch(&self, id: u32) -> Option<Post> {
        let post = sqlx::query_as::<_, Post>("
                SELECT id, title, content, author FROM posts WHERE id = ?
            ")
            .bind(id)
            .fetch_one(&self.pool)
            .await
            .unwrap();

        return Some(post);
    }
    async fn fetch_all(&self) -> Vec<Post> {
        let mut query_builder: QueryBuilder<Sqlite> = QueryBuilder::new("
            SELECT id, title, content, author
            FROM posts
            WHERE 1=1
        ");

        let query = query_builder.build_query_as::<Post>();
        let posts: Vec<Post> = query
            .fetch_all(&self.pool)
            .await
            .unwrap();

        return posts;
    }
    async fn fetch_filtered(&self, filters: PostFilters) -> Vec<Post> {

        let mut query_builder: QueryBuilder<Sqlite> = QueryBuilder::new("
            SELECT id, title, content, author
            FROM posts
            WHERE 1=1
        ");

        let query = query_builder.build_query_as::<Post>();

        let posts: Vec<Post> = query
            .fetch_all(&self.pool)
            .await
            .unwrap();

        return posts;
    }
}
