#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/hello/<name>/<age>")]
fn hello(name: String, age: u8) -> String {
    format!(
        "Hello, human named {}. You have cycled {} times.",
        name, age
    )
}

fn main() {
    rocket::ignite().mount("/", routes![index, hello]).launch();
}
