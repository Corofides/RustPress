use async_std::task;
use sqlx::{
    migrate::MigrateDatabase,
    sqlite::SqlitePoolOptions,
    Sqlite,
    SqlitePool,
    QueryBuilder
};
use crate::{
    Arc,
    Mutex,
    SqliteUserRepository,
    SqlitePostRepository,
    PostRepository,
    Post,
    PostMeta,
    User,
    UserMeta,
};

use crate::repository::{
    SqlitePostmetaRepository,
    UserMetaRepository,
    Repository,
    UserFilters,
    UserMetaFilters,
    PostmetaFilters,
    post_repository::{
        PostFilters,
    },
};

pub trait Database {
    fn new(db_url: &str) -> Self;
    fn post_repository(&self) -> impl Repository<Post, PostFilters> + 'static;
    fn post_meta_repository(&self) -> impl Repository<PostMeta, PostmetaFilters> + 'static;
    fn user_repository(&self) -> impl Repository<User, UserFilters> + 'static;
    fn user_meta_repository(&self) -> impl Repository<UserMeta, UserMetaFilters> + 'static;
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

    fn post_repository(&self) -> impl Repository<Post, PostFilters> + 'static {
        SqlitePostRepository::new(self.pool.clone())
    }

    fn user_repository(&self) -> impl Repository<User, UserFilters> + 'static {
        SqliteUserRepository::new(self.pool.clone())
    }

    fn post_meta_repository(&self) -> impl Repository<PostMeta, PostmetaFilters> + 'static {
        SqlitePostmetaRepository::new(self.pool.clone())
    }

    fn user_meta_repository(&self) -> impl Repository<UserMeta, UserMetaFilters> + 'static {
        UserMetaRepository::new(self.pool.clone())
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
    
}
