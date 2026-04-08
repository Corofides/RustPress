
use crate::structs::post::Post;
use std::sync::{
    Arc,
    Mutex,
};
use serde_aux::field_attributes::deserialize_default_from_empty_object;
use serde_aux::field_attributes::deserialize_number_from_string;
use sqlx::{
    QueryBuilder,
    Sqlite,
    SqlitePool,
    Error,
};
use serde::{
    Deserialize
};
use super::{
    Repository,
    RepositoryError,
};

#[derive(Deserialize, Debug)]
pub struct Pagination {
    #[serde(default = "page_default", deserialize_with = "deserialize_number_from_string")]
    page: u32,
    #[serde(default = "page_size_default", deserialize_with = "deserialize_number_from_string")]
    page_size: u32,
}

fn page_default() -> u32 {
    0
}

fn page_size_default() -> u32 {
    20
}

impl Default for Pagination {
    fn default() -> Self {
        Self {
            page: 0,
            page_size: 0,
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct PostFilters {
    author: Option<u32>,
    title: Option<String>,
    #[serde(flatten)]
    pagination: Pagination,
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
    async fn add(&self, post: Post) -> Result<i64, RepositoryError> {
        let result = sqlx::query("INSERT INTO posts (
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

        match result {
            Ok(result) => {
                return Ok(result.last_insert_rowid());
            },
            Err(_) => {
                return Err(RepositoryError::AddItemError);
            }
        }

    }
    async fn fetch(&self, id: u32) -> Option<Post> {
        let post = sqlx::query_as::<_, Post>("
                SELECT id, title, content, author FROM posts WHERE id = ?
            ")
            .bind(id)
            .fetch_one(&self.pool)
            .await;

        match post {
            Ok(post) => {
                return Some(post);
            }
            Err(_) => {
                return None;
            }
        }
    }
    async fn exists(&self, id: u32) -> bool {
        let exists = sqlx::query("
                SELECT 1 FROM posts WHERE id = ?
            ")
            .bind(id)
            .fetch_one(&self.pool)
            .await;

        match exists {
            Ok(_) => {
                return true;
            }
            Err(_) => {
                return false;
            }
        }
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

        if let Some(author_id) = filters.author {
            query_builder.push(" AND author = ");
            query_builder.push_bind(author_id);
        }

        if let Some(title) = filters.title {
            query_builder.push(" AND title = ");
            query_builder.push_bind(title);
        }

        let query = query_builder.build_query_as::<Post>();

        let posts: Vec<Post> = query
            .fetch_all(&self.pool)
            .await
            .unwrap();

        return posts;
    }
}
