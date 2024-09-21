use super::client::AnthropicClient;
use crate::setters;
use serde::Serialize;
use serde_json::Value;

/// [`TextCompletionsApi`] struct to interact with the text completions endpoint of the API.
pub struct TextCompletionsApi<'a, 'b, 'c>(pub(crate) &'a AnthropicClient<'b, 'c>);

/// Struct representing a request for text completions.
#[derive(Default, Serialize)]
pub struct TextCompletionRequest {
    /// Model name to be used for generating text completions.
    model: String,

    /// Prompt text to initiate the completion.
    prompt: String,

    /// Optional maximum number of tokens to sample in the completion.
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens_to_sample: Option<u64>,

    /// Optional sequences to stop the completion.
    #[serde(skip_serializing_if = "Option::is_none")]
    stop_sequences: Option<Vec<String>>,

    /// Optional sampling temperature.
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f64>,

    /// Optional nucleus sampling parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    top_p: Option<f64>,

    /// Optional number of highest probability tokens to keep for sampling.
    #[serde(skip_serializing_if = "Option::is_none")]
    top_k: Option<u64>,
}

impl TextCompletionRequest {
    /// Create a new instance of [`TextCompletionRequest`].
    ///
    /// # Arguments
    ///
    /// * `model` - The model name to use for generating text completions.
    /// * `prompt` - The prompt string to initiate the completion.
    ///
    /// # Returns
    ///
    /// A new instance of [`TextCompletionRequest`].
    #[inline(always)]
    pub fn new(model: String, prompt: String) -> Self {
        Self {
            model,
            prompt,
            ..Default::default()
        }
    }

    setters! {
        /// Set the maximum number of tokens to sample for the completion.
        ///
        /// # Arguments
        ///
        /// * `max_tokens_to_sample` - The maximum number of tokens to generate.
        ///
        /// # Returns
        ///
        /// The updated instance of [`TextCompletionRequest`].
        max_tokens_to_sample: u64,

        /// Set the sequences at which to stop generating text.
        ///
        /// # Arguments
        ///
        /// * `stop_sequences` - A vector of sequences to stop the text generation.
        ///
        /// # Returns
        ///
        /// The updated instance of [`TextCompletionRequest`].
        stop_sequences: Vec<String>,

        /// Set the temperature for the completion.
        ///
        /// # Arguments
        ///
        /// * `temperature` - Sampling temperature.
        ///
        /// # Returns
        ///
        /// The updated instance of [`TextCompletionRequest`].
        temperature: f64,

        /// Set the nucleus sampling parameter for the completion.
        ///
        /// # Arguments
        ///
        /// * `top_p` - The nucleus sampling parameter.
        ///
        /// # Returns
        ///
        /// The updated instance of [`TextCompletionRequest`].
        top_p: f64,

        /// Set the number of highest probability tokens to keep for sampling.
        ///
        /// # Arguments
        ///
        /// * `top_k` - The number of tokens to consider.
        ///
        /// # Returns
        ///
        /// The updated instance of [`TextCompletionRequest`].
        top_k: u64,
    }
}

impl<'a> TextCompletionsApi<'a, '_, '_> {
    /// Create a text completion using the provided request parameters.
    ///
    /// # Arguments
    ///
    /// * `request` - A [`TextCompletionRequest`] containing the parameters for the completion.
    ///
    /// # Returns
    ///
    /// A [`Result`] containing the JSON response as [`serde_json::Value`] on success,
    /// or a [`serde_json::Error`] on failure.
    pub async fn create(&self, request: TextCompletionRequest) -> reqwest::Result<Value> {
        // Send a POST request to the text completions endpoint with the request body.
        self.0.post("/complete", &request).await
    }
}
