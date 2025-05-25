#[derive(Debug, Clone)]
struct Invitation {
    invitee: String,
    attending: bool,
    message: Option<String>,
}

impl Invitation {
    fn new(invitee: String, attending: bool, message: Option<String>) -> Self {
        Self {
            invitee,
            attending,
            message,
        }
    }
}

fn commercials(hour: u32) -> String {
    match hour {
        0..=7 => "Classic video bundle commercials".to_string(),
        8 | 12 | 18 => "Food commercials".to_string(),
        9..=11 | 13..=17 => "Clothing commercials".to_string(),
        19..=24 => "Season ticket commercials".to_string(),
        _ => "NOT A VALID HOUR".to_string(),
    }
}

fn main() {
    let s1 = String::from("Hello, world!");
    let s2 = String::from("Goodbye, world!");

    let result = longest_string(&s1, &s2);
    println!("The longest string is: {}", result);
}

fn print_string(s: &String) {
    println!("{}", s);
}

fn longest_string<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() { s1 } else { s2 }
}

pub struct User {
    name: String,
}

pub struct UserProfile {
    display_name: String,
}

impl User {
    pub fn new(name: String) -> Self {
        User { name }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn rename(&mut self, new_name: String) {
        self.name = new_name;
    }

    pub fn from_user_profile(profile: &UserProfile) -> Self {
        let display_name = profile.get_display_name();
        User::new(display_name)
    }
}

impl UserProfile {
    pub fn new(display_name: String) -> Self {
        UserProfile { display_name }
    }

    pub fn get_display_name(&self) -> String {
        self.display_name.clone()
    }
}
