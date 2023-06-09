# xata-rs: Lightweight, Strongly Typed Xata Client

xata-rs is a third party Xata API client, allowing to interact with Xata's REST API. 

## Adding xata-rs
```
cargo add xata-rs
```

## API
Create a new client
```
let xata: Xata = Xata::new(
    api_key: "my_key".to_owned(),
    workspace: "workspace_id".to_owned(),
    region: US_EAST_1
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

>> User { "usr_XXXXX", "quanturtle", "my@email.com" }
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