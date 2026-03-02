mod structs;

use structs::author::Author;
use structs::post::Post;
use structs::postmeta::PostMeta;
use structs::usermeta::UserMeta;

fn main() {

    let jane_doe = Author::new(
        "Jane",
        "Doe",
        "JaneDoe32",
    );

    let john_smith = Author::default()
        .set_id(Author::generate_id())
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

    println!("Author: {:?}", jane_doe);
    println!("Author: {:?}", john_smith);
    println!("Post: {:?}", post);
    println!("Postmeta: {:?}", postmeta);
    println!("Usermeta: {:?}", usermeta);
}
