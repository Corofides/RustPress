
use crate::structs::user::User;
use sqlx::{
    Sqlite,
    SqlitePool,
    QueryBuilder,
    Error,
};

pub trait UserRepository {
    fn fetch_all(&self) -> impl Future<Output = Vec<User>>;
    fn create_user(&self, user: &User) -> impl Future<Output = Result<(), Error>>;
}


pub struct SqliteUserRepository<'a> {
    pool: &'a SqlitePool,
}

impl<'a> SqliteUserRepository<'a> {
    pub fn new(pool: &'a SqlitePool) -> Self {
        Self {
            pool
        }
    }
}

impl UserRepository for SqliteUserRepository<'_> {
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
}
