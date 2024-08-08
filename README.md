# Rusty Anthropics Client

A Rust client for interacting with the Anthropic API, featuring examples for messages and text completions. This client utilizes the `dotenv` crate to manage environment variables securely.

## Table of Contents
- [Features](#features)
- [Installation](#installation)
- [Configuration](#configuration)
- [Documentation](#documentation)
- [Usage](#usage)
  - [Messages API Example](#messages-api-example)
  - [Text Completions API Example](#text-completions-api-example)
- [License](#license)

## Features

- Supports the Messages and Text Completions endpoints of the Anthropic API
- Loads API keys from environment variables using the `dotenv` crate
- Handles errors gracefully and returns responses in JSON format

## Installation

To use the Rusty Anthropics Client, follow these steps:

1. Add the required dependencies to your `Cargo.toml`:

```toml
[dependencies]
rusty-anthropic = "0.1.0"
reqwest = { version = "0.12.5", features = ["json"] }
tokio = { version = "1", features = ["full"] }
serde_json = "1.0"
dotenv = "0.15.0"
```

2. Create a `.env` file in the root directory of your project to store your API key:

```env
API_KEY=your_api_key_here
```

## Configuration

The client uses the `dotenv` crate to load environment variables. Ensure that you have a `.env` file in the root directory of your project with the following content:

```env
API_KEY=your_api_key_here
```

This file should be included in your `.gitignore` to prevent your API key from being committed to version control.

## Usage

Below is an example showing how to use the Messages API and the Text Completions API.

### Messages API Example

1. Create a new file named `main.rs` and add the following content:

```rust
use rusty_anthropic::anthropic_api::client::AnthropicClient;
use rusty_anthropic::anthropic_api::messages::{MessagesApi, MessageRequest};
use rusty_anthropic::request_client::RequestClient;
use serde_json::json;
use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables from .env file
    dotenv().ok();

    // Retrieve the API key from the environment variable
    let api_key = env::var("API_KEY").expect("API_KEY environment variable not set");

    // Initialize the RequestClient with your API key
    let request_client = RequestClient::new(api_key);

    // Create an Anthropic client instance
    let base_url = "https://api.anthropic.com/v1";
    let anthropic_client = AnthropicClient::new(&request_client, base_url);

    // Create a Messages API instance
    let messages_api = MessagesApi::new(&anthropic_client);

    // Create a message request
    let model = "claude-3-5-sonnet-20240620".to_string();
    let messages = vec![json!({"role": "user", "content": "Hello, Claude"})];
    let request = MessageRequest::new(model, messages)
        .max_tokens(1024)
        .temperature(1.0);

    // Send the request and get the response
    let response_result = messages_api.create(request).await;

    // Handle and print the response as JSON
    match response_result {
        Ok(response) => {
            println!("{}", response.to_string());
        },
        Err(e) => {
            let error_response = json!({
                "error": e.to_string()
            });
            println!("{}", error_response.to_string());
        }
    }

    Ok(())
}
```

## Documentation

Please visit this link for the documentation (WIP): [here](https://pleaseful.github.io/rusty-anthropic/#)

## License

This project is licensed under the terms of the MIT license. See the [LICENSE](LICENSE.md) file for details.
```