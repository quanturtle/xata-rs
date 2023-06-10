use std::sync::Arc;
use std::collections::HashMap;
use reqwest::blocking::Response;

use crate::error::XataClientError;
use crate::models::{User, Key, NewKey, KeyList, WorkspaceList};
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

    pub fn create_api_key(&self, name: String) -> Result<NewKey, XataClientError> {
        let key_url: String = format!("{}/{}", &self.url, name);
        let resp: Response = self.client._post(&key_url, HashMap::new())?;
        let result: NewKey = self.client._handle_response::<NewKey>(resp)?;
        Ok(result)
    }

    pub fn delete_api_key(&self, name: String) -> Result<(), XataClientError> {
        let key_url: String = format!("{}/{}", &self.url, name);
        let resp: Response = self.client._delete(&key_url)?;
        self.client._handle_response::<()>(resp)?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct Workspaces {
    pub client: Arc<XataClient>,
    pub url: String
}

impl Workspaces {
    pub fn list_workspaces(&self) -> Result<WorkspaceList, XataClientError>{
        let resp: Response = self.client._get(&self.url)?;
        let result: WorkspaceList = self.client._handle_response::<WorkspaceList>(resp)?;
        Ok(result)
    }
}

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