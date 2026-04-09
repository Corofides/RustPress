use serde::Deserialize;
use serde_aux::field_attributes::deserialize_default_from_empty_object;
use serde_aux::field_attributes::deserialize_number_from_string;

#[derive(Deserialize, Debug)]
pub struct Pagination {
    #[serde(default = "Pagination::page_default", deserialize_with = "deserialize_number_from_string")]
    page: u32,
    #[serde(default = "Pagination::page_size_default", deserialize_with = "deserialize_number_from_string")]
    page_size: u32,
}

impl Pagination {
    fn page_default() -> u32 {
        0
    }

    fn page_size_default() -> u32 {
        20
    }
}
