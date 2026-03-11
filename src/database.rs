use async_std::task;
use sqlx::{
    SqlitePool,
    migrate::MigrateDatabase,
    sqlite::SqlitePoolOptions,
    Sqlite,
    Pool,
    QueryBuilder
};
use crate::{
    Post,
    PostMeta,
    User,
    UserMeta,
};
const DB_URL: &str = "sqlite://blog.db";

pub struct Database {
    pool: Pool<Sqlite>,
}

impl Database {

    async fn migrate_db(&mut self) {
        let crate_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();

        let migrations = std::path::Path::new(&crate_dir).join("migrations");

        let migration_result = sqlx::migrate::Migrator::new(migrations)
            .await
            .unwrap()
            .run(&self.pool)
            .await;

        match migration_result {
            Ok(_) => println!("Migration Success!"),
            Err(error) => panic!("Migration Error: {}", error),
        }
    }

    pub fn get_postmeta(&self) -> Vec<PostMeta> {
        task::block_on(async {

            //if let Some(pool) = self.pool.clone() {

                let mut query_builder: QueryBuilder<Sqlite> = QueryBuilder::new("
                    SELECT id, post, key, value
                    FROM postmeta
                    WHERE 1=1
                ");

                let query = query_builder.build_query_as::<PostMeta>();
                let postmeta: Vec<PostMeta> = query
                    .fetch_all(&self.pool)
                    .await
                    .unwrap();

                return postmeta;
            //}

            vec![]
        })
    }

    pub fn get_users(&self) -> Vec<User> {
        task::block_on(async {

            //if let Some(pool) = self.pool.clone() {

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

            //}

            vec![]
        })
    }

    pub fn get_usermeta(&self) -> Vec<UserMeta> {
        task::block_on(async {
            //if let Some(pool) = self.pool.clone() {

                let mut query_builder: QueryBuilder<Sqlite> = QueryBuilder::new("
                    SELECT id, user, key, value,
                    FROM usermeta
                    WHERE 1=1
                ");

                let query = query_builder.build_query_as::<UserMeta>();
                let usermeta: Vec<UserMeta> = query
                    .fetch_all(&self.pool)
                    .await
                    .unwrap();

                return usermeta;
            //}

            vec![]
        })
    }

    pub fn new() -> Self {
        task::block_on(async {
            if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
                println!("Creating DB {}", DB_URL);
                match Sqlite::create_database(DB_URL).await {
                    Ok(_) => println!("Created DB"),
                    Err(error) => panic!("Error: {}", error),
                }
            }

            let pool = SqlitePoolOptions::new()
                .max_connections(5)
                .connect(DB_URL)
                .await;

            if let Ok(pool) = pool {

                let mut new_db = Self {
                    pool: pool
                };

                Self::migrate_db(&mut new_db).await;

                new_db
            } else {
                panic!("Could not start the db!");
            }
        })
    }

    pub fn get_pool(&self) -> &SqlitePool {
        &self.pool
    }
}
