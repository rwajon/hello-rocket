pub fn welcome(name: Option<String>, age: Option<String>) -> String {
    if name != None && age != None {
        format!(
            "Hello, {}, you are {} years old!",
            name.unwrap(),
            age.unwrap()
        )
    } else if name != None {
        format!("Hello, {}!", name.unwrap())
    } else {
        "Hello, World!!!".to_string()
    }
}
