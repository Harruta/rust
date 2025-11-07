use serde::{Deserialize, Serialize};
use serde_json::{self};

#[derive(Serialize, Deserialize, Debug)]
struct Person {
    name: String,
    age: u32,
}

fn main() {
    let person = Person {
        name: String::from("Jhon Doe"),
        age: 32,
    };

    let json_str = serde_json::to_string(&person).unwrap();
    println!("Serialize JSON: {}", json_str);

    let deserialized_person: Person = serde_json::from_str(&json_str).unwrap();
    println!("Deserialize str: {:?}", deserialized_person);
}
