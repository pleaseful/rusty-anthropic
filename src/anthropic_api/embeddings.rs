use crate::anthropic_api::client::AnthropicClient;
use crate::util::insert_optional_param;
use serde_json::{json, Value};

/// EmbeddingsApi struct to interact with the embeddings endpoint of the API.
pub struct EmbeddingsApi<'a> {
    /// Reference to the AnthropicClient.
    client: &'a AnthropicClient<'a>,
}

/// Struct representing a request for embeddings.
pub struct EmbeddingsRequest {
    /// Inputs for which embeddings need to be generated.
    inputs: Vec<String>,
    /// Model name to be used for generating embeddings.
    model: String,
    /// Optional type of the input data.
    input_type: Option<String>,
    /// Optional flag to specify whether to use truncation.
    truncation: Option<bool>,
    /// Optional format for encoding.
    encoding_format: Option<String>,
}

impl EmbeddingsRequest {
    /// Create a new instance of EmbeddingsRequest.
    /// 
    /// # Arguments
    ///
    /// * `model` - The model name to use for generating embeddings.
    /// * `inputs` - A vector of input strings for which embeddings are to be generated.
    ///
    /// # Returns
    ///
    /// A new instance of `EmbeddingsRequest`.
    pub fn new(model: String, inputs: Vec<String>) -> Self {
        EmbeddingsRequest {
            model,
            inputs,
            input_type: None,
            truncation: None,
            encoding_format: None,
        }
    }

    /// Set the input type of the request.
    /// 
    /// # Arguments
    ///
    /// * `input_type` - A string representing the type of inputs.
    ///
    /// # Returns
    ///
    /// The updated instance of `EmbeddingsRequest`.
    pub fn input_type(mut self, input_type: String) -> Self {
        self.input_type = Some(input_type);
        self
    }

    /// Set the truncation flag of the request.
    /// 
    /// # Arguments
    ///
    /// * `truncation` - A boolean flag to enable or disable truncation.
    ///
    /// # Returns
    ///
    /// The updated instance of `EmbeddingsRequest`.
    pub fn truncation(mut self, truncation: bool) -> Self {
        self.truncation = Some(truncation);
        self
    }

    /// Set the encoding format of the request.
    /// 
    /// # Arguments
    ///
    /// * `encoding_format` - A string representing the encoding format.
    ///
    /// # Returns
    ///
    /// The updated instance of `EmbeddingsRequest`.
    pub fn encoding_format(mut self, encoding_format: String) -> Self {
        self.encoding_format = Some(encoding_format);
        self
    }
}

impl<'a> EmbeddingsApi<'a> {
    /// Create a new instance of EmbeddingsApi.
    /// 
    /// # Arguments
    ///
    /// * `client` - A reference to the AnthropicClient.
    ///
    /// # Returns
    ///
    /// A new instance of `EmbeddingsApi`.
    pub fn new(client: &'a AnthropicClient) -> Self {
        EmbeddingsApi { client }
    }

    /// Create embeddings using the provided request parameters.
    ///
    /// # Arguments
    ///
    /// * `request` - An `EmbeddingsRequest` containing the parameters for the embeddings request.
    ///
    /// # Returns
    ///
    /// A `Result` containing the JSON response as `serde_json::Value` on success,
    /// or a boxed error on failure.
    pub async fn create(&self, request: EmbeddingsRequest) -> Result<Value, Box<dyn std::error::Error>> {
        // Construct the full URL for the embeddings endpoint.
        let url = format!("{}/embeddings", self.client.base_url);

        // Initialize a JSON map to build the request body.
        let mut body = serde_json::Map::new();
        
        // Insert required fields to the JSON map.
        body.insert("model".to_string(), json!(request.model));
        body.insert("input".to_string(), json!(request.inputs));

        // Insert optional fields if they are provided.
        insert_optional_param(&mut body, "input_type", request.input_type);
        insert_optional_param(&mut body, "truncation", request.truncation);
        insert_optional_param(&mut body, "encoding_format", request.encoding_format);

        // Send a POST request to the embeddings endpoint with the request body.
        let response = self.client.request_client.post(&url, &Value::Object(body)).await?;
        
        // Parse the JSON response body.
        let json: Value = response.json().await?;
        
        // Return the parsed JSON response.
        Ok(json)
    }
}