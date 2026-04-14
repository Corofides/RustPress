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
use crate::repository::filters::Pagination;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct UserMetaFilters {
    pub user_id: Option<u32>,
    pub key: Option<String>,
    pub value: Option<String>,
    #[serde(flatten)]
    pub pagination: Pagination,
}

impl UserMetaFilters {
    pub fn new() -> Self {
        Self {
            user_id: None,
            key: None,
            value: None,
            pagination: Pagination::default()
        }
    }
    pub fn add_user(mut self, user: &u32) -> Self {
        self.user_id = Some(user.clone());
        self
    }
    pub fn add_key(mut self, key: &str) -> Self {
        self.key = Some(key.to_string());
        self
    }
    pub fn add_value(mut self, value: &str) -> Self {
        self.key = Some(value.to_string());
        self
    }
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

        if let Some(user_id) = filters.user_id {
            query_builder.push(" AND user = ");
            query_builder.push_bind(user_id);
        }

        let query = query_builder.build_query_as::<UserMeta>();
        let meta: Vec<UserMeta> = query
            .fetch_all(&self.pool)
            .await
            .unwrap();

        return meta;
    }
}
