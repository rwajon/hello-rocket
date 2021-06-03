#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

mod api;

fn main() {
    rocket::ignite()
        .mount("/", routes![api::home::index, api::home::home])
        .launch();
}
