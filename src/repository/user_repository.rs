
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

/*pub trait UserRepository {
    fn fetch_all(&self) -> impl Future<Output = Vec<User>>;
    fn create_user(&self, user: &User) -> impl Future<Output = Result<(), Error>>;
}*/

pub struct UserFilters {}

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
        todo!();
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
