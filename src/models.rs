//! Model information types for `/v1/models` and related endpoints.

use serde::{Deserialize, Serialize};

use crate::prelude::*;

/// Model information (OpenAI-compatible minimal format).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Model {
    /// Model ID.
    pub id: String,

    /// Model creation time in Unix timestamp.
    pub created: i64,

    /// The object type, which is always `"model"`.
    pub object: String,

    /// Owner of the model.
    pub owned_by: String,
}

/// Details of a language model.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct LanguageModel {
    /// Model ID.
    pub id: String,

    /// Fingerprint of the xAI system configuration hosting the model.
    pub fingerprint: String,

    /// Creation time of the model in Unix timestamp.
    pub created: i64,

    /// The object type, which is always `"model"`.
    pub object: String,

    /// Owner of the model.
    pub owned_by: String,

    /// Version of the model.
    pub version: String,

    /// The input modalities supported by the model, e.g. `"text"`, `"image"`.
    pub input_modalities: Vec<String>,

    /// The output modalities supported by the model, e.g. `"text"`, `"image"`.
    pub output_modalities: Vec<String>,

    /// Price of the prompt text token in USD cents per 100 million token.
    pub prompt_text_token_price: i64,

    /// Price of a prompt text token that was cached previously.
    pub cached_prompt_text_token_price: i64,

    /// Price of the prompt image token in USD cents per 100 million token.
    pub prompt_image_token_price: i64,

    /// Price of the completion text token in USD cents per 100 million token.
    pub completion_text_token_price: i64,

    /// Price of the search in USD cents per 100 million searches.
    pub search_price: i64,

    /// Alias ID(s) of the model that user can use in a request's model field.
    pub aliases: Vec<String>,
}

/// Details of an embedding model.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct EmbeddingModel {
    /// Model ID.
    pub id: String,

    /// Fingerprint of the xAI system configuration hosting the model.
    pub fingerprint: String,

    /// Model creation time in Unix timestamp.
    pub created: i64,

    /// Object type, should be model.
    pub object: String,

    /// Owner of the model.
    pub owned_by: String,

    /// Version of the model.
    pub version: String,

    /// The input modalities supported by the model.
    pub input_modalities: Vec<String>,

    /// The output modalities supported by the model.
    #[serde(default)]
    pub output_modalities: Vec<String>,

    /// Price of the prompt text token in USD cents per million token.
    pub prompt_text_token_price: i64,

    /// Price of the prompt image token in USD cents per million token.
    pub prompt_image_token_price: i64,

    /// Alias ID(s) of the model that user can use in a request's model field.
    pub aliases: Vec<String>,
}

/// Details of an image generation model.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ImageGenerationModel {
    /// Model ID.
    pub id: String,

    /// Fingerprint of the xAI system configuration hosting the model.
    pub fingerprint: String,

    /// Maximum prompt length.
    pub max_prompt_length: i64,

    /// Model creation time in Unix timestamp.
    pub created: i64,

    /// The object type, which is always `"model"`.
    pub object: String,

    /// Owner of the model.
    pub owned_by: String,

    /// Version of the model.
    pub version: String,

    /// The input modalities supported by the model.
    #[serde(default)]
    pub input_modalities: Vec<String>,

    /// The output modalities supported by the model.
    #[serde(default)]
    pub output_modalities: Vec<String>,

    /// Price of a single image in USD cents.
    pub image_price: i64,

    /// Alias ID(s) of the model that user can use in a request's model field.
    pub aliases: Vec<String>,
}

/// Response listing all models.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ListModelsResponse {
    /// A list of models with minimalized information.
    pub data: Vec<Model>,

    /// The object type of `data` field, which is always `"list"`.
    pub object: String,
}

/// Response listing all language models.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ListLanguageModelsResponse {
    /// Array of available language models.
    pub models: Vec<LanguageModel>,
}

/// Response listing all embedding models.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ListEmbeddingModelsResponse {
    /// Array of available embedding models.
    pub models: Vec<EmbeddingModel>,
}

/// Response listing all image generation models.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ListImageGenerationModelsResponse {
    /// Array of available image generation models.
    pub models: Vec<ImageGenerationModel>,
}
