use serde::{Deserialize, Serialize};

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