#[derive(Debug, Default)]
pub struct Post {
    id: usize,
    title: String,
    content: String,
    author: usize,
}

impl Post {

    pub fn generate_id() -> usize {
        0
    }

    pub fn new(title: &str, content: &str, author: usize) -> Self {
        Self::default()
            .set_id(Self::generate_id())
            .set_title(title)
            .set_content(content)
            .set_author(author)
    }

    pub fn id(&self) -> &usize {
        &self.id
    }

    pub fn set_id(mut self, id: usize) -> Self {
        self.id = id;
        self
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn set_title(mut self, title: &str) -> Self {
        self.title = title.to_string();
        self
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn set_content(mut self, content: &str) -> Self {
        self.content = content.to_string();
        self
    }

    pub fn author(&self) -> &usize {
        &self.author
    }

    pub fn set_author(mut self, author: usize) -> Self {
        self.author = author;
        self
    }

}
