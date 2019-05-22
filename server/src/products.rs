pub struct Client;

use crate::api;

impl Client { 
    pub fn new() -> Client { 
        Client{}
    }

    pub fn all_videos(&self) -> Vec<&Product> {
        return vec!(&Product{}, &Product{})
    }
}

/// the root query type
pub struct Query;

pub struct Product;

#[juniper::object(
    Context = api::Context,
)]
impl Product{
    pub fn  hello() -> String {
        "world".to_string()
    }
}