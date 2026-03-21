use crate::Database;
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

/*pub struct PostService<T: Repository<Post, PostFilters>> {
    repository: T,
}*/

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
}
