# Pipedrive API Client

This crate provides a client for the Pipedrive API, generated from a combination of OpenAPI specifications.

## Authentication

To use the Pipedrive API, you need to provide an API key. You can do this by creating a `Configuration` object and setting the `api_key` field.

```rust
use pipedrive::v2::apis::configuration::{ApiKey, Configuration};

let mut config = Configuration::new();
config.api_key = Some(ApiKey {
    prefix: None,
    key: "YOUR_API_KEY".to_string(),
});
```

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
pipedrive = { path = "." }
tokio = { version = "1", features = ["full"] }
```

Here is an example of how to get the current user:

```rust
use pipedrive::v1::apis::users_api;
use pipedrive::v1::apis::configuration::{ApiKey, Configuration};

#[tokio::main]
async fn main() {
    let mut config = Configuration::new();
    config.api_key = Some(ApiKey {
        prefix: None,
        key: std::env::var("PIPEDRIVE_API_KEY").expect("PIPEDRIVE_API_KEY not set"),
    });

    let user = users_api::get_current_user(&config).await;

    match user {
        Ok(user) => println!("Successfully fetched user: {:?}", user),
        Err(e) => eprintln!("Error fetching user: {}", e),
    }
}
```

## Handling Custom Fields

This API client provides robust support for Pipedrive's custom fields feature, allowing you to create, update, and retrieve custom field data for various resources.

### Creating and Updating with Custom Fields

When creating or updating a resource (such as a person, deal, or organization), you can include custom field data in the request body. The `custom_fields` property should be a map where the keys are the custom field hashes and the values are the data you want to set.

Here’s how you can add a new person with custom field data:

```rust
use pipedrive::v2::apis::persons_api;
use pipedrive::v2::models::AddPersonRequest;
use std::collections::HashMap;
use serde_json::json;

// ... (inside an async function)

let mut custom_fields = HashMap::new();
custom_fields.insert(
    "YOUR_CUSTOM_FIELD_HASH".to_string(),
    json!("Custom field value"),
);

let person_request = AddPersonRequest {
    name: Some("John Doe".to_string()),
    custom_fields: Some(custom_fields),
    ..Default::default()
};

let new_person = persons_api::add_person(&config, Some(person_request)).await;

match new_person {
    Ok(person) => println!("Successfully created person: {:?}", person),
    Err(e) => eprintln!("Error creating person: {}", e),
}
```

### Fetching Custom Fields

When retrieving a resource, you can specify which custom fields to include in the response by providing their hashes in the `custom_fields` parameter.

This example shows how to fetch a person and include a specific custom field:

```rust
use pipedrive::v2::apis::persons_api;

// ... (inside an async function)

let person = persons_api::get_person(&config, 1, None, Some("YOUR_CUSTOM_FIELD_HASH")).await;

match person {
    Ok(person) => println!("Successfully fetched person with custom fields: {:?}", person),
    Err(e) => eprintln!("Error fetching person: {}", e),
}
```
## API Versions

This client supports two versions of the Pipedrive API, which can be enabled via features in your `Cargo.toml`:

- `v1`: The stable version of the API.
- `v2`: The beta version of the API (enabled by default).

To use a specific version, enable the corresponding feature:

```toml
[dependencies]
pipedrive = { path = ".", features = ["v1"] }
```

## API Reference

The full documentation for this crate, including all API endpoints and models, can be found in the `docs` directory.

- [v1 Documentation](./docs/v1)
- [v2 Documentation](./docs/v2)

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please feel free to submit a pull request.