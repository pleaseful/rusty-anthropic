use crate::request_client::RequestClient;

/// AnthropicClient struct to interact with the Anthropic API.
pub struct AnthropicClient<'a> {
    /// Reference to the HTTP client used for making API requests.
    pub request_client: &'a RequestClient,
    
    /// Base URL for the Anthropic API.
    pub base_url: &'a str,
}

impl<'a> AnthropicClient<'a> {
    /// Create a new instance of AnthropicClient.
    /// 
    /// # Arguments
    ///
    /// * `request_client` - A reference to the HTTP client.
    /// * `base_url` - The base URL for the Anthropic API.
    ///
    /// # Returns
    ///
    /// A new instance of `AnthropicClient`.
    pub fn new(request_client: &'a RequestClient, base_url: &'a str) -> Self {
        AnthropicClient {
            request_client,
            base_url,
        }
    }
}