#![feature(proc_macro_hygiene, decl_macro)]

// externals
use rocket::response::content;
use rocket::State;

// local module declarations
mod playground;
mod api;
mod products;

#[rocket::get("/")]
fn playground() -> content::Html<&'static str> {
    content::Html(playground::PLAYGROUND_CONTENT)
}

#[rocket::post("/", data = "<request>")]
fn api(
    request: juniper_rocket::GraphQLRequest,
    schema: State<api::Schema>,
) -> juniper_rocket::GraphQLResponse {
    // we want to create a new collection of dataloaders on every request
    let context = api::Context::new();

    // resolve the request given the schema and current context
    request.execute(&schema, &context)
}

fn main() {
    rocket::ignite()
        .manage(api::root_node())
        .mount("/", rocket::routes![playground, api])
        .launch();
}
