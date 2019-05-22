#![feature(proc_macro_hygiene, decl_macro)]

// externals
use rocket::response::content;
use juniper::{EmptyMutation, RootNode};
use rocket::State;

// local module declarations
mod playground;
mod schema;

type Schema = RootNode<'static, schema::Query, EmptyMutation<schema::Context>>;

#[rocket::get("/")]
fn playground() -> content::Html<&'static str> {
    content::Html(playground::PLAYGROUND_CONTENT)
}

#[rocket::post("/", data = "<request>")]
fn api(
    context: State<schema::Context>,
    request: juniper_rocket::GraphQLRequest,
    schema: State<Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute(&schema, &context)
}

fn main() {
    rocket::ignite().manage(())
        .manage(Schema::new(schema::Query, EmptyMutation::<schema::Context>::new())).mount("/", rocket::routes![playground, api]).launch();
}
