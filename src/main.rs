mod client;
mod error;
mod models;
mod api;

use client::{Xata, XataClient};
use error::XataClientError;
use models::User;
use api::Users;

use std::collections::HashMap;


fn main() {
    // low level API
    let xata_cli: XataClient = XataClient::new("your_api_key".to_owned());
    let resp = xata_cli._get("https://api.xata.io/user").unwrap();
    let result = xata_cli._handle_response::<User>(resp).unwrap();
    println!("{:?}", result);
    
    // high level API
    // let xata: Xata = Xata::new("your_api_key".to_owned());
    let xata: Xata = Xata::from_env(); // requires .env file

    // get example
    let user: User = xata.users.get_user_details().unwrap();
    println!("{:?}", user);

    // put example
    let mut payload: HashMap<String, String> = HashMap::new();
    payload.insert("email".to_owned(), "your_email".to_owned()); // replace "your_email" with your email
    payload.insert("fullname".to_owned(), "another name".to_owned());
    let updated_user: User = xata.users.update_user_info(payload).unwrap();

    println!("{:?}", updated_user);
}
