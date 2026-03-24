pub mod posts;
pub mod users;

pub trait Router {
    fn get_routes() -> Vec<(String, String)>;
    fn router() -> String;
    fn add_route() -> ();
}
