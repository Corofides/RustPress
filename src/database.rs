use async_std::task;
use sqlx::{
    migrate::MigrateDatabase,
    sqlite::SqlitePoolOptions,
    Sqlite,
    Pool,
    QueryBuilder
};
use crate::Post;

const DB_URL: &str = "sqlite://blog.db";

pub struct Database {
    pool: Option<Pool<Sqlite>>,
}

impl Database {

    async fn migrate_db(&mut self) {
        if let Some(pool) = self.pool.clone() {

            let crate_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();

            let migrations = std::path::Path::new(&crate_dir).join("migrations");

            let migration_result = sqlx::migrate::Migrator::new(migrations)
                .await
                .unwrap()
                .run(&pool)
                .await;

            match migration_result {
                Ok(_) => println!("Migration Success!"),
                Err(error) => panic!("Migration Error: {}", error),
            }
        }
    }

    pub fn get_posts(&self) -> Vec<Post> {
        task::block_on(async {
            if let Some(pool) = self.pool.clone() {

                let mut query_builder: QueryBuilder<Sqlite> = QueryBuilder::new("
                    SELECT id, title, content, author
                    FROM posts
                    WHERE 1=1
                ");

                let query = query_builder.build_query_as::<Post>();
                let posts: Vec<Post> = query
                    .fetch_all(&pool)
                    .await
                    .unwrap();

                return posts;
            }

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
                    pool: Some(pool)
                };

                Self::migrate_db(&mut new_db).await;

                new_db
            } else {
                panic!("Could not start the db!");
            }
        })
    }
}
