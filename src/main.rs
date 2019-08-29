#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world2!"
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}
