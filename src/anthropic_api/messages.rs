use super::client::AnthropicClient;
use crate::setters;
use serde::Serialize;
use serde_json::Value;

/// [`MessagesApi`] struct to interact with the messages endpoint of the API.
pub struct MessagesApi<'a, 'b, 'c>(pub(crate) &'a AnthropicClient<'b, 'c>);

/// Struct representing a request to send messages.
#[derive(Default, Serialize)]
pub struct MessageRequest {
    /// Model name to be used for generating messages.
    model: String,

    /// History of messages in the conversation.
    messages: Vec<Value>,

    /// Optional maximum number of tokens to generate.
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<u64>,

    /// Optional sampling temperature.
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f64>,

    /// Optional sequences to stop generation.
    #[serde(skip_serializing_if = "Option::is_none")]
    stop_sequences: Option<Vec<String>>,

    /// Optional flag for whether to stream back partial progress.
    #[serde(skip_serializing_if = "Option::is_none")]
    stream: Option<bool>,
}

impl MessageRequest {
    /// Create a new instance of [`MessageRequest`].
    ///
    /// # Arguments
    ///
    /// * `model` - The model name to use for generating messages.
    /// * `messages` - A vector of previous messages in the conversation.
    ///
    /// # Returns
    ///
    /// A new instance of [`MessageRequest`].
    #[inline(always)]
    pub fn new(model: String, messages: Vec<Value>) -> Self {
        Self {
            model,
            messages,
            ..Default::default()
        }
    }

    setters! {
        /// Set the maximum number of tokens for the response.
        ///
        /// # Arguments
        ///
        /// * `max_tokens` - The maximum number of tokens to generate.
        ///
        /// # Returns
        ///
        /// The updated instance of [`MessageRequest`].
        max_tokens: u64,

        /// Set the temperature for the response.
        ///
        /// # Arguments
        ///
        /// * `temperature` - Sampling temperature.
        ///
        /// # Returns
        ///
        /// The updated instance of [`MessageRequest`].
        temperature: f64,

        /// Set the stop sequences for the response.
        ///
        /// # Arguments
        ///
        /// * `stop_sequences` - A vector of sequences at which to stop the response.
        ///
        /// # Returns
        ///
        /// The updated instance of [`MessageRequest`].
        stop_sequences: Vec<String>,

        /// Set the stream flag for the response.
        ///
        /// # Arguments
        ///
        /// * `stream` - A boolean flag indicating whether to stream partial results.
        ///
        /// # Returns
        ///
        /// The updated instance of [`MessageRequest`].
        stream: bool,
    }
}

impl<'a> MessagesApi<'a, '_, '_> {
    /// Send a message request using the provided parameters.
    ///
    /// # Arguments
    ///
    /// * `request` - A [`MessageRequest`] containing the parameters for the messages request.
    ///
    /// # Returns
    ///
    /// A [`Result`] containing the JSON response as [`serde_json::Value`] on success,
    /// or a [`serde_json::Error`] on failure.
    pub async fn create(&self, request: MessageRequest) -> reqwest::Result<Value> {
        // Send a POST request to the messages endpoint with the request body.
        self.0.post("/messages", &request).await
    }
}
