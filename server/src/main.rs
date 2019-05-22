#![feature(proc_macro_hygiene, decl_macro)]


#[rocket::get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    rocket::ignite().mount("/", rocket::routes![index]).launch();
}