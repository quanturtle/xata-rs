use serde::{Deserialize, Serialize};

enum Region {
    US_EAST_1,
    US_EAST_2,
    US_WEST_1,
    US_WEST_2
}

impl std::fmt::Display for Region {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Region::US_EAST_1 => write!(f, "us-east-1"),
            Region::US_EAST_2 => write!(f, "us-east-2"),
            Region::US_WEST_1 => write!(f, "us-west-1"),
            Region::US_WEST_2 => write!(f, "us-west-2"),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    id: String,
    fullname: String,
    email: String,
}


#[derive(Debug, Deserialize, Serialize)]
pub struct Key {
    name: String, 
    #[serde(rename = "createdAt")]
    created_at: String
}


#[derive(Debug, Deserialize, Serialize)]
pub struct KeyList {
    keys: Vec<Key>,
}