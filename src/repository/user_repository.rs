use crate::repository::filters::Pagination;
use crate::structs::user::User;
use sqlx::{
    Sqlite,
    SqlitePool,
    QueryBuilder,
    Error,
};
use super::{
    Repository,
    RepositoryError,
};
use serde::Deserialize;

/*pub trait UserRepository {
    fn fetch_all(&self) -> impl Future<Output = Vec<User>>;
    fn create_user(&self, user: &User) -> impl Future<Output = Result<(), Error>>;
}*/

#[derive(Deserialize)]
pub struct UserFilters {
    first_name: Option<String>,
    last_name: Option<String>,
    display_name: Option<String>,
    pagination: Pagination,
}

pub struct SqliteUserRepository {
    pool: SqlitePool,
}

impl SqliteUserRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self {
            pool
        }
    }
}

impl Repository<User, UserFilters> for SqliteUserRepository {
    async fn add(&self, item: User) -> Result<i64, RepositoryError> {
        let result = sqlx::query("INSERT INTO user (
                    first_name, last_name, display_name
                ) VALUES (
                    ?,
                    ?,
                    ?
                )
            ")
            .bind(item.first_name())
            .bind(item.last_name())
            .bind(item.display_name())
            .execute(&self.pool)
            .await;

        match result {
            Ok(result) => {
                return Ok(result.last_insert_rowid());
            },
            Err(_) => {
                return Err(RepositoryError::AddItemError);
            },
        }
    }
    async fn exists(&self, id: u32) -> bool {
        let result = sqlx::query("
                SELECT 1 FROM user WHERE id = ?
            ")
            .bind(id)
            .execute(&self.pool)
            .await;

        match result {
            Ok(result) => {
                return true;
            },
            Err(_) => {
                return false;
            }
        }
    }
    async fn fetch(&self, id: u32) -> Option<User> {
        let user = sqlx::query_as::<_, User>("
                SELECT id, first_name, last_name, display_name FROM user WHERE id = ?
            ")
            .bind(id)
            .fetch_one(&self.pool)
            .await
            .unwrap();

        return Some(user);
    }
    async fn fetch_all(&self) -> Vec<User> {
        let mut query_builder: QueryBuilder<Sqlite> = QueryBuilder::new("
            SELECT id, first_name, last_name, display_name 
            FROM user 
            WHERE 1=1
        ");

        let query = query_builder.build_query_as::<User>();
        let users: Vec<User> = query
            .fetch_all(&self.pool)
            .await
            .unwrap();

        return users;
    }
    async fn fetch_filtered(&self, filters: UserFilters) -> Vec<User> {
        let mut query_builder: QueryBuilder<Sqlite> = QueryBuilder::new("
            SELECT id, first_name, last_name, display_name 
            FROM user 
            WHERE 1=1
        ");

        if let Some(first_name) = filters.first_name {
            query_builder.push(" AND first_name = ");
            query_builder.push_bind(first_name);
        }

        if let Some(last_name) = filters.last_name {
            query_builder.push(" AND last_name = ");
            query_builder.push_bind(last_name);
        }

        if let Some(display_name) = filters.display_name {
            query_builder.push(" AND display_name = ");
            query_builder.push_bind(display_name);
        }

        query_builder.push(" LIMIT = ");
        query_builder.push_bind(filters.pagination.page_size);

        query_builder.push(" OFFSET = ");
        query_builder.push_bind(filters.pagination.offset());


        let query = query_builder.build_query_as::<User>();
        let users: Vec<User> = query
            .fetch_all(&self.pool)
            .await
            .unwrap();

        return users;

    }

}

/*impl UserRepository for SqliteUserRepository<'_> {
    async fn fetch_all(&self) -> Vec<User> {
        let mut query_builder: QueryBuilder<Sqlite> = QueryBuilder::new("
            SELECT id, first_name, last_name, display_name 
            FROM user 
            WHERE 1=1
        ");

        let query = query_builder.build_query_as::<User>();
        let users: Vec<User> = query
            .fetch_all(self.pool)
            .await
            .unwrap();

        return users;
    }
    async fn create_user(&self, user: &User) -> Result<(), Error> {
        let _ = sqlx::query("INSERT INTO user (
                    first_name, last_name, display_name
                ) VALUES (
                    ?,
                    ?,
                    ?
                )
            ")
            .bind(user.first_name())
            .bind(user.last_name())
            .bind(user.display_name())
            .execute(self.pool)
            .await;

        Ok(())
    }
} */
