mod structs;

#[derive(Debug)]
pub struct Post {
    id: usize,
    title: String,
    content: String,
    author: usize,
}

#[derive(Debug)]
pub struct PostMeta {
    id: usize,
    post: usize,
    key: String,
    value: String,
}


