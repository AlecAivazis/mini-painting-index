#![feature(proc_macro_hygiene, decl_macro)]

// externals
use rocket::response::content;

// local module declarations
mod playground;

#[rocket::get("/")]
fn playground() -> content::Html<&'static str> {
    content::Html(playground::PLAYGROUND_CONTENT)
}

fn main() {
    rocket::ignite().mount("/", rocket::routes![playground]).launch();
}
