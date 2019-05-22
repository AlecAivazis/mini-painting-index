// external crates
use juniper::{EmptyMutation, RootNode};

use super::products;

/// the root query type
pub struct Query;
#[juniper::object(
    Context = Context,
)]
impl Query {
    /// the version of the platform
    fn apiVersion() -> &'static str {
        // grab the current cargo version with a macro
        option_env!("CARGO_PKG_VERSION").unwrap_or("unknown")
    }

    /// the list of product videos that we know of
    fn productVideos(context: &Context) -> Vec<&products::Product> {
        context.products.all_videos()
    }
}

/// the context type for queries
pub struct Context {
    products: products::Client,
}
// Mark the Database as a valid context type for Juniper
impl juniper::Context for Context {}

impl Context {
    pub fn new() -> Context {
        // creating a new context involves instantiatin each domain-specific client
        Context {
            products: products::Client::new(),
        }
    }
}

/// the root schema type
pub type Schema = RootNode<'static, Query, EmptyMutation<Context>>;


/// return the root node representing our schema
pub fn root_node() -> Schema {
    Schema::new(Query, EmptyMutation::<Context>::new())
}
