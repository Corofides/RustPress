mod structs;
mod idgenerator;

use structs::user::User;
use structs::post::Post;
use structs::postmeta::PostMeta;
use structs::usermeta::UserMeta;
use idgenerator::IdGenerator;

mod database;
use crate::database::{Database};

#[tokio::main]
async fn main() {

    let database = Database::new();

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
