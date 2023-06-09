use std::sync::Arc;
use std::collections::HashMap;
use reqwest::blocking::Response;

use crate::error::XataClientError;
use crate::models::{User, Key, KeyList};
use crate::client::XataClient;


#[derive(Debug)]
pub struct Users {
    pub client: Arc<XataClient>,
    pub url: String
}

impl Users {
    pub fn get_user_details(&self) -> Result<User, XataClientError> {
        let resp: Response = self.client._get(&self.url)?;
        let result: User = self.client._handle_response::<User>(resp)?;
        Ok(result)
    }

    pub fn update_user_info(&self, payload: HashMap<String, String>) -> Result<User, XataClientError> {
        let resp: Response = self.client._put(&self.url, payload)?;
        let result: User = self.client._handle_response::<User>(resp)?;
        Ok(result)
    }

    pub fn delete_user(&self) -> Result<(), XataClientError> {
        let resp: Response = self.client._delete(&self.url)?;
        self.client._handle_response::<()>(resp)?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct Authentication {
    pub client: Arc<XataClient>,
    pub url: String
}

impl Authentication {
    pub fn get_api_keys(&self) -> Result<KeyList, XataClientError> {
        let resp: Response = self.client._get(&self.url)?;
        let result: KeyList = self.client._handle_response::<KeyList>(resp)?;
        Ok(result)
    }
}

pub struct Workspaces {}
impl Workspaces {}

pub struct Invites {}
impl Invites {}

pub struct Databases {}
impl Databases {}

pub struct Branch {}
impl Branch {}

pub struct Migrations {}
impl Migrations {}

pub struct Tables {}
impl Tables {}

pub struct Records {}
impl Records {}

pub struct Query {}
impl Query {}