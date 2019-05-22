// external crates
use juniper::{EmptyMutation, RootNode};

// the root query type
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
}

// the context type for queries
pub struct Context;

impl Context {
    pub fn new() -> Context {
        Context {}
    }
}

// the root schema type
pub type Schema = RootNode<'static, Query, EmptyMutation<Context>>;

pub fn create_schema() -> Schema {
    Schema::new(Query, EmptyMutation::<Context>::new())
}
