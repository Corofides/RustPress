use sqlx::{
    Sqlite,
    QueryBuilder,
    SqlitePool
};
use super::{
    Repository,
    RepositoryError,
};
use crate::structs::postmeta::PostMeta;

pub struct PostmetaFilters {
    pub post: Option<u32>,
    pub key: Option<String>,
    pub value: Option<String>,
}

impl PostmetaFilters {
    pub fn new() -> Self {
        return Self {
            post: None,
            key: None,
            value: None,
        }
    }
    pub fn set_post(mut self, post: &u32) -> Self {
        self.post = Some(post.clone());
        self
    }
    pub fn set_key(mut self, key: &str) -> Self {
        self.key = Some(key.to_string());
        self
    }
    pub fn set_value(mut self, value: &str) -> Self {
        self.value = Some(value.to_string());
        self
    }

}

pub struct SqlitePostmetaRepository {
    pool: SqlitePool
}

impl SqlitePostmetaRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self {
            pool
        }
    }
}

impl Repository<PostMeta, PostmetaFilters> for SqlitePostmetaRepository {
    async fn add(&self, item: PostMeta) -> Result<i64, RepositoryError> {
        let result = sqlx::query("INSERT INTO postmeta (
                    post, key, value 
                ) VALUES (
                    ?,
                    ?,
                    ?
                )
            ")
            .bind(item.post())
            .bind(item.key())
            .bind(item.value())
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
    async fn fetch(&self, id: u32) -> Option<PostMeta> {
        let post_meta = sqlx::query_as::<_, PostMeta>("
                SELECT id, post, key, value FROM postmeta WHERE id = ?
            ")
            .bind(id)
            .fetch_one(&self.pool)
            .await;

        match post_meta {
            Ok(post_meta) => {
                return Some(post_meta);
            }
            Err(_) => {
                return None;
            }
        }
    }
    async fn fetch_all(&self) -> Vec<PostMeta> {
        let mut query_builder: QueryBuilder<Sqlite> = QueryBuilder::new("
            SELECT id, post, key, value 
            FROM postmeta 
            WHERE 1=1
        ");

        let query = query_builder.build_query_as::<PostMeta>();
        let meta: Vec<PostMeta> = query
            .fetch_all(&self.pool)
            .await
            .unwrap();

        return meta;
    }
    async fn fetch_filtered(&self, filters: PostmetaFilters) -> Vec<PostMeta> {
        let mut query_builder: QueryBuilder<Sqlite> = QueryBuilder::new("
            SELECT id, post, key, value 
            FROM postmeta 
            WHERE 1=1
        ");

        if let Some(post_id) = filters.post {
            query_builder.push(" AND post = ");
            query_builder.push_bind(post_id);
        }

        if let Some(key) = filters.key {
            query_builder.push(" AND key = ");
            query_builder.push_bind(key);
        }

        if let Some(value) = filters.value {
            query_builder.push(" AND value = ");
            query_builder.push_bind(value);
        }

        let query = query_builder.build_query_as::<PostMeta>();
        let meta: Vec<PostMeta> = query
            .fetch_all(&self.pool)
            .await
            .unwrap();

        return meta;

    }
}
