//! Tokenization API types for `/v1/tokenize-text` endpoint.

use serde::{Deserialize, Serialize};

use crate::prelude::*;

/// Request body for tokenization.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct TokenizeRequest {
    /// The model to tokenize with.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,

    /// The text content to be tokenized.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,

    /// Optional user identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

/// Response body for tokenization.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct TokenizeResponse {
    /// A list of tokens.
    pub token_ids: Vec<TokenizeResponseToken>,
}

/// A token from the tokenization response.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct TokenizeResponseToken {
    /// The integer representation of the token for the model.
    pub token_id: u32,

    /// The string of the token.
    pub string_token: String,

    /// The bytes that constituted the token.
    pub token_bytes: Vec<u32>,
}
