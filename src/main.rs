mod structs;
mod idgenerator;
mod repository;
mod service;

use structs::user::User;
use structs::post::Post;
use structs::postmeta::PostMeta;
use structs::usermeta::UserMeta;
use idgenerator::IdGenerator;
use repository::post_repository::SqlitePostRepository;
use service::post_service::PostService;

mod database;
use crate::{
    repository::post_repository::PostRepository,
    database::{Database},
};

#[tokio::main]
async fn main() {

    let database = Database::new();

    let post_repository = SqlitePostRepository::new(database.get_pool());
    let post_service = PostService::new(post_repository);

    let posts = post_service.get_posts();

    println!("{:?}", posts);


    let id_generator = IdGenerator::default();

    let jane_doe = User::new(
        "Jane",
        "Doe",
        "JaneDoe32",
    );

    let john_smith = User::default()
        .set_id(User::generate_id())
        .set_first_name("John")
        .set_last_name("Smith")
        .set_display_name("JSmith99");

    let post = Post::new(
        "Hello Blog",
        "this is a post with some content",
        0
    );

    //post_repository.create_post(&post).await;

    let postmeta = PostMeta::new(
        0,
        "subtitle",
        "The first post"
    );

    let usermeta = UserMeta::new(
        0,
        "position",
        "Content Manager"
    );

    println!("User: {:?}", jane_doe);
    println!("User: {:?}", john_smith);
    println!("Post: {:?}", post);
    println!("Postmeta: {:?}", postmeta);
    println!("Usermeta: {:?}", usermeta);
}
