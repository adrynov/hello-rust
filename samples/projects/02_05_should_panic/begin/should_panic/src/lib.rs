pub struct Person {
    name: String
}

impl Person {
    pub fn new(name: String) -> Person {
        if name.is_empty() {
            panic!("A Person needs a name!");
        }

        Person{name}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn person_test() {
        let new_person = Person::new("Steve".to_string());
        assert_eq!(new_person.name, "Steve".to_string());
    }
}