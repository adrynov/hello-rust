use std::fmt::{self, Display, Formatter};

#[derive(PartialEq, PartialOrd)]
pub struct Satellite {
    pub id: String,
    pub name: String,
    pub description: String,
    pub velocity: f64,
}

pub struct SpaceStation {
    pub id: String,
    pub name: String,
    pub description: String,
    pub crew_size: u8,
    pub altitude: f64,
}

trait Description {
    fn describe(&self) -> String;
}

impl Description for Satellite {
    fn describe(&self) -> String {
        format!(
            "Satellite ID: {}, Name: {}, Description: {}, Velocity: {} km/h",
            self.id, self.name, self.description, self.velocity
        )
    }
}

impl Display for Satellite {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Satellite ID: {}, Name: {}, Description: {}, Velocity: {} km/h",
            self.id, self.name, self.description, self.velocity
        )
    }
}
