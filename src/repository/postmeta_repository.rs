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

pub struct PostmetaFilters {}

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
    async fn add(&self, item: PostMeta) -> Result<(), RepositoryError> {
        let _ = sqlx::query("INSERT INTO postmeta (
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
        Ok(())
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
            FROM post_meta 
            WHERE 1=1
        ");

        let query = query_builder.build_query_as::<PostMeta>();
        let meta: Vec<PostMeta> = query
            .fetch_all(&self.pool)
            .await
            .unwrap();

        return meta;

    }
}
