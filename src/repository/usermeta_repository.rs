use sqlx::{
    Sqlite,
    SqlitePool,
    QueryBuilder,
};
use super::{
    Repository,
    RepositoryError
};
use crate::structs::usermeta::UserMeta;

pub enum UserMetaFilters {
}

pub struct UserMetaRepository {
    pool: SqlitePool
}

impl UserMetaRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self {
            pool
        }
    }
}

impl Repository<UserMeta, UserMetaFilters> for UserMetaRepository {
    async fn add(&self, item: UserMeta) -> Result<i64, RepositoryError> {
        let result = sqlx::query("INSERT INTO usermeta (
                    user, key, value 
                ) VALUES (
                    ?,
                    ?,
                    ?
                )
            ")
            .bind(item.user())
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
    async fn exists(&self, id: u32) -> bool {
        let result = sqlx::query("
                SELECT 1 FROM usermeta WHERE id = ?
            ")
            .bind(id)
            .execute(&self.pool)
            .await;

        match result {
            Ok(_) => {
                return true;
            },
            Err(_) => {
                return false;
            }
        }
    }
    async fn fetch(&self, id: u32) -> Option<UserMeta> {
        let user_meta = sqlx::query_as::<_, UserMeta>("
                SELECT id, user, key, value FROM user_meta WHERE id = ?
            ")
            .bind(id)
            .fetch_one(&self.pool)
            .await
            .unwrap();

        return Some(user_meta);
    }

    async fn fetch_all(&self) -> Vec<UserMeta> {
        let mut query_builder: QueryBuilder<Sqlite> = QueryBuilder::new("
            SELECT id, user, key, value 
            FROM user_meta 
            WHERE 1=1
        ");

        let query = query_builder.build_query_as::<UserMeta>();
        let meta: Vec<UserMeta> = query
            .fetch_all(&self.pool)
            .await
            .unwrap();

        return meta;
    }

    async fn fetch_filtered(&self, filters: UserMetaFilters) -> Vec<UserMeta> {
        let mut query_builder: QueryBuilder<Sqlite> = QueryBuilder::new("
            SELECT id, user, key, value 
            FROM user_meta 
            WHERE 1=1
        ");

        let query = query_builder.build_query_as::<UserMeta>();
        let meta: Vec<UserMeta> = query
            .fetch_all(&self.pool)
            .await
            .unwrap();

        return meta;
    }
}
