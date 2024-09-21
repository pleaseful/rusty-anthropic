use super::{
    embeddings::EmbeddingsApi, messages::MessagesApi, text_completions::TextCompletionsApi,
};
use reqwest::Client;
use serde::{de::DeserializeOwned, Serialize};

/// [`AnthropicClient`] struct to interact with the Anthropic API.
pub struct AnthropicClient<'a, 'b> {
    /// The HTTP client used for making API requests.
    client: Client,

    /// The base URL for the Anthropic API.
    base_url: &'a str,

    /// The API key used for interacting with the API.
    api_key: &'b str,
}

impl<'a, 'b> AnthropicClient<'a, 'b> {
    /// Create a new instance of [`AnthropicClient`].
    ///
    /// # Arguments
    ///
    /// * `base_url` - The base URL for the Anthropic API.
    /// * `api_key` - The API key.
    ///
    /// # Returns
    ///
    /// A new instance of [`AnthropicClient`].
    #[inline(always)]
    pub fn new(base_url: &'a str, api_key: &'b str) -> Self {
        Self {
            client: Client::new(),
            base_url,
            api_key,
        }
    }

    pub async fn post<B: Serialize + ?Sized, T: DeserializeOwned>(
        &self,
        url: &str,
        body: &B,
    ) -> reqwest::Result<T> {
        self.client
            .post(format!("{}{url}", self.base_url))
            .header("content-type", "application/json")
            .header("x-api-key", self.api_key)
            .header("anthropic-version", "2023-06-01")
            .json(body)
            .send()
            .await?
            .json()
            .await
    }

    pub const fn embeddings(&self) -> EmbeddingsApi {
        EmbeddingsApi(self)
    }

    pub const fn messages(&self) -> MessagesApi {
        MessagesApi(self)
    }

    pub const fn text_completions(&self) -> TextCompletionsApi {
        TextCompletionsApi(self)
    }
}
