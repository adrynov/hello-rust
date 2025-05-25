use std::fmt::Display;

#[derive(PartialEq, PartialOrd)]
pub struct Satellite {
    pub id: String,
    pub name: String,
    pub description: String,
    pub velocity: f64,
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
