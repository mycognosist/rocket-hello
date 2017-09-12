#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/location")]
fn country() -> &'static str {
    "South Africa."
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index,country])
        .launch();
}
