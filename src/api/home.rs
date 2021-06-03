use rocket::response::content;

#[path = "../controllers/mod.rs"]
mod controllers;

#[get("/?<name>&<age>")]
pub fn index(name: Option<String>, age: Option<String>) -> content::Json<String> {
    controllers::welcome::welcome(name, age)
}

#[get("/home?<name>&<age>")]
pub fn home(name: Option<String>, age: Option<String>) -> content::Json<String> {
    controllers::welcome::welcome(name, age)
}
