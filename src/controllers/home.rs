extern crate serde_json;

use rocket::response::content;

#[derive(Debug, Serialize, Deserialize)]
struct Response {
    status: i32,
    message: String,
}

pub fn index(name: Option<String>, age: Option<String>) -> content::Json<String> {
    let mut response = Response {
        status: 200,
        message: format!("Hello, World!"),
    };

    if name != None && age != None {
        response = Response {
            status: 200,
            message: format!(
                "Hello, {}, you are {} years old!",
                name.unwrap(),
                age.unwrap()
            ),
        };
    } else if name != None {
        response = Response {
            status: 200,
            message: format!("Hello, {}!", name.unwrap()),
        };
    }
    content::Json(serde_json::to_string(&response).unwrap())
}
