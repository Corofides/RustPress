use async_std::task;
use sqlx::{
    migrate::MigrateDatabase,
    sqlite::SqlitePoolOptions,
    Sqlite,
    SqlitePool,
    QueryBuilder
};
use crate::{
    SqliteUserRepository,
    SqlitePostRepository,
    PostRepository,
    Post,
    PostMeta,
    User,
    UserMeta,
};

use crate::repository::{
    Repository,
    post_repository::{
        PostFilters,
    },
};
use crate::repository::user_repository::UserRepository;

pub trait Database {
    fn new(db_url: &str) -> Self;
    fn post_repository(&self) -> impl Repository<Post, PostFilters>;
    fn user_repository(&self) -> impl UserRepository;
}

pub struct SqliteDatabase {
    pool: SqlitePool,
}

impl Database for SqliteDatabase {
    
    fn new(db_url: &str) -> Self {
        task::block_on(async {
            if !Sqlite::database_exists(db_url).await.unwrap_or(false) {
                
                match Sqlite::create_database(db_url).await {
                    Ok(_) => println!("Created DB"),
                    Err(error) => panic!("Error: {}", error),
                }
            }

            let pool = SqlitePoolOptions::new()
                .max_connections(5)
                .connect(db_url)
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

    fn post_repository(&self) -> impl Repository<Post, PostFilters> {
        SqlitePostRepository::new(&self.pool)
    }

    fn user_repository(&self) -> impl UserRepository {
        SqliteUserRepository::new(&self.pool)
    }
}

impl SqliteDatabase {

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
}
