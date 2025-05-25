use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Id(u64);

#[derive(Serialize, Deserialize)]
pub struct User {
    id: Id,
    username: String,
}
