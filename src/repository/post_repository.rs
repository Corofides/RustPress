
use crate::structs::post::Post;
use sqlx::{
    QueryBuilder,
    Sqlite,
    SqlitePool,
    Error,
};

pub trait PostRepository {
    fn fetch_all(&self) -> impl Future<Output = Vec<Post>>;
    fn create_post(&self, post: &Post) -> impl Future<Output = Result<(), Error>>;
}

pub struct SqlitePostRepository<'a> {
    pool: &'a SqlitePool,
}

impl<'a> SqlitePostRepository<'a> {
    pub fn new(pool: &'a SqlitePool) -> Self {
        Self {
            pool
        }
    }
}

impl PostRepository for SqlitePostRepository<'_> {

    async fn create_post(&self, post: &Post) -> Result<(), Error> {

        let _ = sqlx::query("INSERT INTO posts (
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
            .execute(self.pool)
            .await;

        Ok(())

    }

    async fn fetch_all(&self) -> Vec<Post> {

        let mut query_builder: QueryBuilder<Sqlite> = QueryBuilder::new("
            SELECT id, title, content, author
            FROM posts
            WHERE 1=1
        ");

        let query = query_builder.build_query_as::<Post>();
        let posts: Vec<Post> = query
            .fetch_all(self.pool)
            .await
            .unwrap();

        return posts;
    }
}


