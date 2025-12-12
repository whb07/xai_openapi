//! Embedding API types for `/v1/embeddings` endpoint.

use serde::{Deserialize, Serialize};

use crate::prelude::*;

use crate::usage::EmbeddingUsage;

/// Embedding request body.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct EmbeddingRequest {
    /// Input text to embed, encoded as a string or list of tokens.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<EmbeddingInput>,

    /// ID of the model to use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,

    /// The format to return the embeddings in. Can be either `float` or `base64`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding_format: Option<String>,

    /// The number of dimensions the resulting output embeddings should have.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<i32>,

    /// Flag to use the new format of the API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preview: Option<bool>,

    /// A unique identifier representing your end-user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

/// Input for embedding request.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EmbeddingInput {
    /// A string to be embedded.
    String(String),
    /// An array of strings to be embedded.
    StringArray(Vec<String>),
    /// A token in integer to be embedded.
    Ints(Vec<i32>),
    /// An array of tokens in integers to be embedded.
    IntsArray(Vec<Vec<i32>>),
}

impl Default for EmbeddingInput {
    fn default() -> Self {
        EmbeddingInput::String(String::new())
    }
}

/// Embedding response.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct EmbeddingResponse {
    /// The object type of `data` field, which is always `"list"`.
    pub object: String,

    /// Model ID used to create embedding.
    pub model: String,

    /// A list of embedding objects.
    pub data: Vec<Embedding>,

    /// Token usage information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<EmbeddingUsage>,
}

/// An embedding object.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Embedding {
    /// Index of the embedding object in the data list.
    pub index: i32,

    /// Embedding content.
    pub embedding: EmbeddingContent,

    /// The object type, which is always `"embedding"`.
    pub object: String,
}

/// Embedding content.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EmbeddingContent {
    /// Embedding as an array of floats.
    Float(Vec<f32>),
    /// Embedding in base64 string.
    Base64(String),
}

impl Default for EmbeddingContent {
    fn default() -> Self {
        EmbeddingContent::Float(Vec::new())
    }
}
