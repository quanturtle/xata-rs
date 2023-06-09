# xata-rs: Lightweight, Strongly Typed Xata Client

xata-rs is a third party Xata API client, allowing interaction with Xata's REST API. 

## Adding xata-rs (WIP)
```
cargo add xata-rs
```

## API
Create a new client
```
let xata: Xata = Xata::new(
    api_key: "my_key".to_owned(),
    workspace: "workspace_id".to_owned(),
    region: Region::US_EAST_1
);
```
or from env variables
```
let xata: Xata = Xata::from_env();
```

Call an endpoint (for more info, check the [xata docs](https://xata.io/docs/overview))

```
let user: User = xata.users.get_details()?;
println!("{#?}", user)
```
```
> User { "usr_XXXXX", "quanturtle", "my@email.com" }
```

```
let mut payload: HashMap<String, String> = HashMap::new();
payload.insert("email".to_owned(), "your_email".to_owned());
payload.insert("fullname".to_owned(), "another name".to_owned());

let updated_user: User = xata.users.update_user_info(payload).unwrap();

println!("{#?}", updated_user)
```
```
> User { "usr_XXXXX", "another name", "my@email.com" }
```

## API Endpoints
`Users`  
`Authentication`  
`Workspaces`  
`Invites`  
`Databases`  
`Branch`  
`Migrations`  
`Table`  


## Errors
The enum `XataClientError` contains all errors that could arise from the use of the client. To map `XataClientError` to your application, implement `From<>`:
```
impl From<XataClientError> for MyApplicatioError {
    fn from(error: XataClientError) -> Self {
        MyApplicationError::MyCustomError(error)
    }
}
```

## TODO
* Add `async` support with `tokio` for `POST` requests
* support for `datetime` objects with `chronos`
* `query` endpoint and `QueryBuilder`