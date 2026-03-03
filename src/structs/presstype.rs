#[derive(Eq, Hash, PartialEq)]
pub enum PressType {
    Post,
    PostMeta,
    User,
    UserMeta,
    Term,
    TermTaxonomy,
}
