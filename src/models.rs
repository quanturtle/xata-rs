use std::str::FromStr;
use serde::{Deserialize, Serialize};

use crate::{error::XataClientError, client::Xata};

#[derive(Debug)]
pub enum Region {
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

impl std::str::FromStr for Region {
    type Err = XataClientError;

    fn from_str(s: &str) -> Result<Region, XataClientError> {
        match s {
            "us-east-1" => Ok(Region::US_EAST_1),
            "us-east-2" => Ok(Region::US_EAST_2),
            "us-west-1" => Ok(Region::US_WEST_1),
            "us-west-2" => Ok(Region::US_WEST_2),
            _ => Err(XataClientError::InvalidRegion),
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
pub struct NewKey {
    name: String,
    key: String, 
    #[serde(rename = "createdAt")]
    created_at: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct KeyList {
    keys: Vec<Key>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Workspace {
    id: String,
    slug: String,
    name: String,
    role: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct WorkspaceList {
    workspaces: Vec<Workspace>,
}