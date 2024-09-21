use super::client::AnthropicClient;
use crate::setters;
use serde::Serialize;
use serde_json::Value;

/// [`EmbeddingsApi`] struct to interact with the embeddings endpoint of the API.
pub struct EmbeddingsApi<'a, 'b, 'c>(pub(crate) &'a AnthropicClient<'b, 'c>);

/// Struct representing a request for embeddings.
#[derive(Default, Serialize)]
pub struct EmbeddingsRequest {
    /// Inputs for which embeddings need to be generated.
    inputs: Vec<String>,

    /// Model name to be used for generating embeddings.
    model: String,

    /// Optional type of the input data.
    #[serde(skip_serializing_if = "Option::is_none")]
    input_type: Option<String>,

    /// Optional flag to specify whether to use truncation.
    #[serde(skip_serializing_if = "Option::is_none")]
    truncation: Option<bool>,

    /// Optional format for encoding.
    #[serde(skip_serializing_if = "Option::is_none")]
    encoding_format: Option<String>,
}

impl EmbeddingsRequest {
    /// Create a new instance of [`EmbeddingsRequest`].
    ///
    /// # Arguments
    ///
    /// * `model` - The model name to use for generating embeddings.
    /// * `inputs` - A vector of input strings for which embeddings are to be generated.
    ///
    /// # Returns
    ///
    /// A new instance of [`EmbeddingsRequest`].
    #[inline(always)]
    pub fn new(model: String, inputs: Vec<String>) -> Self {
        Self {
            model,
            inputs,
            ..Default::default()
        }
    }

    setters! {
        /// Set the input type of the request.
        ///
        /// # Arguments
        ///
        /// * `input_type` - A string representing the type of inputs.
        ///
        /// # Returns
        ///
        /// The updated instance of [`EmbeddingsRequest`].
        input_type: String,

        /// Set the truncation flag of the request.
        ///
        /// # Arguments
        ///
        /// * `truncation` - A boolean flag to enable or disable truncation.
        ///
        /// # Returns
        ///
        /// The updated instance of [`EmbeddingsRequest`].
        truncation: bool,

        /// Set the encoding format of the request.
        ///
        /// # Arguments
        ///
        /// * `encoding_format` - A string representing the encoding format.
        ///
        /// # Returns
        ///
        /// The updated instance of [`EmbeddingsRequest`].
        encoding_format: String,
    }
}

impl<'a> EmbeddingsApi<'a, '_, '_> {
    /// Create embeddings using the provided request parameters.
    ///
    /// # Arguments
    ///
    /// * `request` - An [`EmbeddingsRequest`] containing the parameters for the embeddings request.
    ///
    /// # Returns
    ///
    /// A [`Result`] containing the JSON response as [`serde_json::Value`] on success,
    /// or a [`serde_json::Error`] on failure.
    pub async fn create(&self, request: EmbeddingsRequest) -> reqwest::Result<Value> {
        // Send a POST request to the embeddings endpoint with the request body.
        self.0.post("/embeddings", &request).await
    }
}
