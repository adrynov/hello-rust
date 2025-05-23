use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ViewCount {
    #[serde(rename = "N")]
    pub n: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Person {
    pub name: String,
    pub age: u8,
    pub phones: Vec<String>,
    pub viewCount: ViewCount,
}

impl Person {
    pub fn from_json(data: &str) -> Result<Person, serde_json::Error> {
        serde_json::from_str(data)
    }
}
