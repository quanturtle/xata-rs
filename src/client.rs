use crate::api::{Users, Authentication, Workspaces};
use crate::error::XataClientError;
use crate::models::{User, Key, KeyList, Region};

use dotenv::dotenv;
use std::str::FromStr;
use std::collections::HashMap;
use std::sync::Arc;

use serde::de::DeserializeOwned;

use reqwest::StatusCode;
use reqwest::blocking::{Client, Response, RequestBuilder};
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};


#[derive(Debug)]
pub struct XataClient {
    pub client: Arc<Client>,
    pub api_key: String,
}

impl XataClient {
    pub fn new(api_key: String) -> XataClient {
        let client: Arc<Client> = Arc::new(Client::new());
        XataClient { client, api_key }
    }

    pub fn _set_header(&self, request: RequestBuilder) -> RequestBuilder {
        request.header(AUTHORIZATION, &format!("Bearer {}", self.api_key))
            .header(CONTENT_TYPE, "application/json")
    }

    pub fn _get(&self, url: &str) -> Result<Response, reqwest::Error> {
        let build_req: RequestBuilder = self.client.get(url);
        let req: RequestBuilder = self._set_header(build_req);
        req.send()
    }

    pub fn _post(&self, url: &str, payload: HashMap<String, String>) -> Result<Response, reqwest::Error> {
        let build_req: RequestBuilder = self.client.post(url);
        let req: RequestBuilder = self._set_header(build_req);
        req.json(&payload).send()
    }

    pub fn _put(&self, url: &str, payload: HashMap<String, String>) -> Result<Response, reqwest::Error> {
        let build_req: RequestBuilder = self.client.put(url);
        let req: RequestBuilder = self._set_header(build_req);
        req.json(&payload).send()
    }

    pub fn _delete(&self, url: &str) -> Result<Response, reqwest::Error> {
        let build_req: RequestBuilder = self.client.delete(url);
        let req: RequestBuilder = self._set_header(build_req);
        req.send()
    }

    pub fn parse<'de, T>(&self, response_body: String) -> Result<T, serde_json::Error>
    where T: DeserializeOwned {
            let parsed: T = serde_json::from_str(&response_body)?;
            Ok(parsed)
    }

    pub fn _handle_response<'de, T>(&self, response: Response) -> Result<T, XataClientError> 
    where T: DeserializeOwned {
        let status: StatusCode = response.status();
        match status {
            // 200 OK / 201 Created
            StatusCode::OK | StatusCode::CREATED | StatusCode::NO_CONTENT => {
                let response_body: String = response.text()?;
                let parsed: T = self.parse::<T>(response_body)?;
                Ok(parsed)
            },
            // 400 Bad Request
            StatusCode::BAD_REQUEST => Err(XataClientError::BadRequest),
            // 401 Unauthorized
            StatusCode::UNAUTHORIZED => Err(XataClientError::InvalidAPIKey),
            // 403 Forbidden
            StatusCode::FORBIDDEN => Err(XataClientError::Forbidden),
            // 404 Not Found
            StatusCode::NOT_FOUND => Err(XataClientError::NotFound),
            // 422
            StatusCode::UNPROCESSABLE_ENTITY => Err(XataClientError::InvalidDatabaseURL),
            // 423
            StatusCode::LOCKED => Err(XataClientError::RateLimitExceeded),
            // 500 and above
            code if code.is_server_error() => Err(XataClientError::ServerError),
            // Everything else
            _ => Err(XataClientError::Generic)
        }
    } 
}

#[derive(Debug)]
pub struct Xata {
    pub users: Users,
    pub authentication: Authentication,
    pub workspaces: Workspaces,
}

impl Xata {
    pub fn new(api_key: String, workspace: String, region: Region) -> Xata {
        let xata_client: Arc<XataClient> = Arc::new(XataClient::new(api_key));
        
        let users: Users = Users { 
            client: Arc::clone(&xata_client),
            url: "https://api.xata.io/user".to_owned() 
        };
        let authentication: Authentication = Authentication { 
            client: Arc::clone(&xata_client),
            url: "https://api.xata.io/user/keys".to_owned()
        };
        let workspaces: Workspaces = Workspaces {
            client: Arc::clone(&xata_client),
            url: "https://api.xata.io/workspaces".to_owned()
        }; 
        
        Xata { users, authentication, workspaces }
    }

    pub fn from_env() -> Xata {
        let env_vars: HashMap<String, String> = Self::load_env_vars();

        let api_key: String = env_vars.get("API_KEY").expect("API_KEY not found").to_owned();
        let workspace: String = env_vars.get("WORKSPACE").expect("WORKSPACE not found").to_owned();
        let region: Region = Region::from_str(env_vars.get("REGION").expect("REGION not found")).expect("Invalid region");

        Xata::new(api_key, workspace, region)        
    }

    fn load_env_vars() -> HashMap<String, String> {
        dotenv().ok();
        let env_vars: HashMap<String, String> = std::env::vars().collect();
        env_vars
    }
}

// mod tests {
//     #[test]
//     fn test() {
//         assert_eq!(1, 1);
//     }
// }