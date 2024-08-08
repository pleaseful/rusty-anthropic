use crate::anthropic_api::client::AnthropicClient;
use crate::util::insert_optional_param;
use serde_json::{json, Value};

/// MessagesApi struct to interact with the messages endpoint of the API.
pub struct MessagesApi<'a> {
    /// Reference to the AnthropicClient.
    client: &'a AnthropicClient<'a>,
}

/// Struct representing a request to send messages.
pub struct MessageRequest {
    /// Model name to be used for generating messages.
    model: String,
    /// History of messages in the conversation.
    messages: Vec<Value>,
    /// Optional maximum number of tokens to generate.
    max_tokens: Option<u64>,
    /// Optional sampling temperature.
    temperature: Option<f64>,
    /// Optional sequences to stop generation.
    stop_sequences: Option<Vec<String>>,
    /// Optional flag for whether to stream back partial progress.
    stream: Option<bool>,
}

impl MessageRequest {
    /// Create a new instance of MessageRequest.
    /// 
    /// # Arguments
    ///
    /// * `model` - The model name to use for generating messages.
    /// * `messages` - A vector of previous messages in the conversation.
    ///
    /// # Returns
    ///
    /// A new instance of `MessageRequest`.
    pub fn new(model: String, messages: Vec<Value>) -> Self {
        MessageRequest {
            model,
            messages,
            max_tokens: None,
            temperature: None,
            stop_sequences: None,
            stream: None,
        }
    }

    /// Set the maximum number of tokens for the response.
    /// 
    /// # Arguments
    ///
    /// * `max_tokens` - The maximum number of tokens to generate.
    ///
    /// # Returns
    ///
    /// The updated instance of `MessageRequest`.
    pub fn max_tokens(mut self, max_tokens: u64) -> Self {
        self.max_tokens = Some(max_tokens);
        self
    }

    /// Set the temperature for the response.
    /// 
    /// # Arguments
    ///
    /// * `temperature` - Sampling temperature.
    ///
    /// # Returns
    ///
    /// The updated instance of `MessageRequest`.
    pub fn temperature(mut self, temperature: f64) -> Self {
        self.temperature = Some(temperature);
        self
    }

    /// Set the stop sequences for the response.
    /// 
    /// # Arguments
    ///
    /// * `stop_sequences` - A vector of sequences at which to stop the response.
    ///
    /// # Returns
    ///
    /// The updated instance of `MessageRequest`.
    pub fn stop_sequences(mut self, stop_sequences: Vec<String>) -> Self {
        self.stop_sequences = Some(stop_sequences);
        self
    }

    /// Set the stream flag for the response.
    /// 
    /// # Arguments
    ///
    /// * `stream` - A boolean flag indicating whether to stream partial results.
    ///
    /// # Returns
    ///
    /// The updated instance of `MessageRequest`.
    pub fn stream(mut self, stream: bool) -> Self {
        self.stream = Some(stream);
        self
    }
}

impl<'a> MessagesApi<'a> {
    /// Create a new instance of MessagesApi.
    /// 
    /// # Arguments
    ///
    /// * `client` - A reference to the AnthropicClient.
    ///
    /// # Returns
    ///
    /// A new instance of `MessagesApi`.
    pub fn new(client: &'a AnthropicClient) -> Self {
        MessagesApi { client }
    }

    /// Send a message request using the provided parameters.
    ///
    /// # Arguments
    ///
    /// * `request` - A `MessageRequest` containing the parameters for the messages request.
    ///
    /// # Returns
    ///
    /// A `Result` containing the JSON response as `serde_json::Value` on success,
    /// or a boxed error on failure.
    pub async fn create(&self, request: MessageRequest) -> Result<Value, Box<dyn std::error::Error>> {
        // Construct the full URL for the messages endpoint.
        let url = format!("{}/messages", self.client.base_url);

        // Initialize a JSON map to build the request body.
        let mut body = serde_json::Map::new();
        
        // Insert required fields into the JSON map.
        body.insert("model".to_string(), json!(request.model));
        body.insert("messages".to_string(), json!(request.messages));

        // Insert optional fields if they are provided.
        insert_optional_param(&mut body, "max_tokens", request.max_tokens);
        insert_optional_param(&mut body, "temperature", request.temperature);
        insert_optional_param(&mut body, "stop_sequences", request.stop_sequences.map(|s| Value::Array(s.into_iter().map(Value::String).collect())));
        insert_optional_param(&mut body, "stream", request.stream);

        // Send a POST request to the messages endpoint with the request body.
        let response = self.client.request_client.post(&url, &Value::Object(body)).await?;
        
        // Parse the JSON response body.
        let json: Value = response.json().await?;
        
        // Return the parsed JSON response.
        Ok(json)
    }
}