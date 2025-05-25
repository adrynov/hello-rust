mod models;

use models::person::Person;

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
            let view_count = person.view_count.n;
            println!("View count: {}", view_count);
            // Example usage:
            // println!("Please call {} at the number {}", person.name, person.phones[0]);
        }
        Err(e) => {
            eprintln!("Failed to parse person: {}", e);
        }
    }
}
