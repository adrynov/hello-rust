mod models;

use serde::{Deserialize, Serialize};
use models::person::{Person, ViewCount};

const DATA: &str = r#"
{
    "name": "John Doe",
    "age": 43,
    "phones": [
        "+44 1234567",
        "+44 2345678"
    ],
    "viewCount": {
        "N": 1
    }
}"#;

fn main() {
    match Person::from_json(DATA) {
        Ok(person) => {
            println!("Parsed person: {:?}", person);
            let view_count = person.viewCount.n;
            println!("View count: {}", view_count);
            // Example usage:
            // println!("Please call {} at the number {}", person.name, person.phones[0]);
        }
        Err(e) => {
            eprintln!("Failed to parse person: {}", e);
        }
    }

    let sum_squares: i32 = (1..=10).map(|x| x * x).sum();
}
