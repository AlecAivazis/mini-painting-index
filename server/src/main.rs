#![feature(proc_macro_hygiene, decl_macro)]

// externals
use rocket::response::content;
use rocket::State;

// local module declarations
mod playground;
mod schema;

#[rocket::get("/")]
fn playground() -> content::Html<&'static str> {
    content::Html(playground::PLAYGROUND_CONTENT)
}

#[rocket::post("/", data = "<request>")]
fn api(
    context: State<schema::Context>,
    request: juniper_rocket::GraphQLRequest,
    schema: State<schema::Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute(&schema, &context)
}

fn main() {
    rocket::ignite()
        .manage(schema::Context)
        .manage(schema::create_schema())
        .mount("/", rocket::routes![playground, api])
        .launch();
}
