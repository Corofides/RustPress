use crate::Database;
use crate::User;
use crate::UserFilters;
use crate::UserMeta;
use crate::UserMetaFilters;
use crate::database::SqliteDatabase;
use crate::repository::UserMetaRepository;
use crate::UserMetaService;
use crate::PostService;
use crate::PostRepository;
use crate::PostMetaService;
use crate::UserService;
use crate::Post;
use crate::Repository;
use crate::PostFilters;
use crate::PostMeta;
use crate::PostmetaFilters;

pub struct ServiceProvider {
    database: SqliteDatabase,
}

impl ServiceProvider {
    pub fn new(database: SqliteDatabase) -> Self {
        Self {
            database
        }
    }
    pub fn post_service(&self) -> PostService<impl Repository<Post, PostFilters>> {
        let post_repository = self.database.post_repository();
        let post_service = PostService::new(post_repository);
        post_service
    }
    pub fn user_service(&self) -> UserService<impl Repository<User, UserFilters>> {
        let repository = self.database.user_repository();
        let user_service = UserService::new(repository);
        user_service
    }
    pub fn user_meta_service(&self) -> UserMetaService<impl Repository<UserMeta, UserMetaFilters>> {
        let repository = self.database.user_meta_repository();
        let user_meta_service = UserMetaService::new(repository);
        user_meta_service
    }
    pub fn post_meta_service(&self) -> PostMetaService<impl Repository<PostMeta, PostmetaFilters>> {
        let repository = self.database.post_meta_repository();
        let post_meta_service = PostMetaService::new(repository);
        post_meta_service
    }
}
