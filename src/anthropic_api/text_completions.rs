use crate::anthropic_api::client::AnthropicClient;
use crate::util::{insert_optional_param, strings_to_json_array};
use serde_json::{json, Value};

/// TextCompletionsApi struct to interact with the text completions endpoint of the API.
pub struct TextCompletionsApi<'a> {
    /// Reference to the AnthropicClient.
    client: &'a AnthropicClient<'a>,
}

/// Struct representing a request for text completions.
pub struct TextCompletionRequest {
    /// Model name to be used for generating text completions.
    model: String,
    /// Prompt text to initiate the completion.
    prompt: String,
    /// Optional maximum number of tokens to sample in the completion.
    max_tokens_to_sample: Option<u64>,
    /// Optional sequences to stop the completion.
    stop_sequences: Option<Vec<String>>,
    /// Optional sampling temperature.
    temperature: Option<f64>,
    /// Optional nucleus sampling parameter.
    top_p: Option<f64>,
    /// Optional number of highest probability tokens to keep for sampling.
    top_k: Option<u64>,
}

impl TextCompletionRequest {
    /// Create a new instance of TextCompletionRequest.
    /// 
    /// # Arguments
    ///
    /// * `model` - The model name to use for generating text completions.
    /// * `prompt` - The prompt string to initiate the completion.
    ///
    /// # Returns
    ///
    /// A new instance of `TextCompletionRequest`.
    pub fn new(model: String, prompt: String) -> Self {
        TextCompletionRequest {
            model,
            prompt,
            max_tokens_to_sample: None,
            stop_sequences: None,
            temperature: None,
            top_p: None,
            top_k: None,
        }
    }

    /// Set the maximum number of tokens to sample for the completion.
    /// 
    /// # Arguments
    ///
    /// * `max_tokens_to_sample` - The maximum number of tokens to generate.
    ///
    /// # Returns
    ///
    /// The updated instance of `TextCompletionRequest`.
    pub fn max_tokens_to_sample(mut self, max_tokens_to_sample: u64) -> Self {
        self.max_tokens_to_sample = Some(max_tokens_to_sample);
        self
    }

    /// Set the sequences at which to stop generating text.
    /// 
    /// # Arguments
    ///
    /// * `stop_sequences` - A vector of sequences to stop the text generation.
    ///
    /// # Returns
    ///
    /// The updated instance of `TextCompletionRequest`.
    pub fn stop_sequences(mut self, stop_sequences: Vec<String>) -> Self {
        self.stop_sequences = Some(stop_sequences);
        self
    }

    /// Set the temperature for the completion.
    /// 
    /// # Arguments
    ///
    /// * `temperature` - Sampling temperature.
    ///
    /// # Returns
    ///
    /// The updated instance of `TextCompletionRequest`.
    pub fn temperature(mut self, temperature: f64) -> Self {
        self.temperature = Some(temperature);
        self
    }

    /// Set the nucleus sampling parameter for the completion.
    /// 
    /// # Arguments
    ///
    /// * `top_p` - The nucleus sampling parameter.
    ///
    /// # Returns
    ///
    /// The updated instance of `TextCompletionRequest`.
    pub fn top_p(mut self, top_p: f64) -> Self {
        self.top_p = Some(top_p);
        self
    }

    /// Set the number of highest probability tokens to keep for sampling.
    /// 
    /// # Arguments
    ///
    /// * `top_k` - The number of tokens to consider.
    ///
    /// # Returns
    ///
    /// The updated instance of `TextCompletionRequest`.
    pub fn top_k(mut self, top_k: u64) -> Self {
        self.top_k = Some(top_k);
        self
    }
}

impl<'a> TextCompletionsApi<'a> {
    /// Create a new instance of TextCompletionsApi.
    /// 
    /// # Arguments
    ///
    /// * `client` - A reference to the AnthropicClient.
    ///
    /// # Returns
    ///
    /// A new instance of `TextCompletionsApi`.
    pub fn new(client: &'a AnthropicClient) -> Self {
        TextCompletionsApi { client }
    }

    /// Create a text completion using the provided request parameters.
    ///
    /// # Arguments
    ///
    /// * `request` - A `TextCompletionRequest` containing the parameters for the completion.
    ///
    /// # Returns
    ///
    /// A `Result` containing the JSON response as `serde_json::Value` on success,
    /// or a boxed error on failure.
    pub async fn create(&self, request: TextCompletionRequest) -> Result<Value, Box<dyn std::error::Error>> {
        // Construct the full URL for the text completions endpoint.
        let url = format!("{}/complete", self.client.base_url);

        // Initialize a JSON map to build the request body.
        let mut body = serde_json::Map::new();
        
        // Insert required fields into the JSON map.
        body.insert("model".to_string(), json!(request.model));
        body.insert("prompt".to_string(), json!(request.prompt));

        // Insert optional fields if they are provided.
        insert_optional_param(&mut body, "max_tokens_to_sample", request.max_tokens_to_sample);
        insert_optional_param(&mut body, "stop_sequences", request.stop_sequences.map(|s| strings_to_json_array(&s)));
        insert_optional_param(&mut body, "temperature", request.temperature);
        insert_optional_param(&mut body, "top_p", request.top_p);
        insert_optional_param(&mut body, "top_k", request.top_k);

        // Send a POST request to the text completions endpoint with the request body.
        let response = self.client.request_client.post(&url, &Value::Object(body)).await?;
        
        // Parse the JSON response body.
        let json: Value = response.json().await?;
        
        // Return the parsed JSON response.
        Ok(json)
    }
}