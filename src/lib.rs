//! Rust types for the xAI API.
//!
//! This crate provides type definitions for interacting with the xAI API,
//! including support for Grok models via chat completions, embeddings,
//! image generation, and more.
//!
//! # Overview
//!
//! The types in this crate are organized by API endpoint:
//!
//! - [`chat`] - Chat completions API (`/v1/chat/completions`)
//! - [`responses`] - Responses API (`/v1/responses`)
//! - [`embeddings`] - Embeddings API (`/v1/embeddings`)
//! - [`images`] - Image generation API (`/v1/images/generations`)
//! - [`models`] - Models API (`/v1/models`, `/v1/language-models`, etc.)
//! - [`messages`] - Anthropic-compatible messages API (`/v1/messages`)
//! - [`search`] - Document search API (`/v1/documents/search`)
//! - [`tokenize`] - Tokenization API (`/v1/tokenize-text`)
//!
//! # Features
//!
//! - `std` (default) - Enable std library support. Uses `std::collections::HashMap`.
//! - When `std` is disabled (`no_std`), uses `hashbrown::HashMap` and requires `alloc`.
//!
//! # Example
//!
//! ```ignore
//! use xai_openapi::chat::{ChatRequest, Message};
//!
//! let request = ChatRequest {
//!     model: Some("grok-3".to_string()),
//!     messages: vec![Message::User {
//!         content: "Hello!".into(),
//!         name: None,
//!     }],
//!     ..Default::default()
//! };
//! ```

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc;

mod prelude;

pub mod chat;
pub mod common;
pub mod embeddings;
pub mod images;
pub mod messages;
pub mod models;
pub mod responses;
pub mod search;
pub mod tokenize;
pub mod tools;
pub mod usage;

// Re-export HashMap based on feature flag
#[cfg(feature = "std")]
pub use std::collections::HashMap;

#[cfg(not(feature = "std"))]
pub use hashbrown::HashMap;

// Re-export commonly used types at crate root for convenience
pub use chat::{ChatRequest, ChatResponse, Message};
pub use common::{Content, ImageUrl, StreamOptions};
pub use embeddings::{EmbeddingRequest, EmbeddingResponse};
pub use images::{GenerateImageRequest, GeneratedImageResponse};
pub use messages::{MessageRequest, MessageResponse};
pub use models::{LanguageModel, ListModelsResponse, Model};
pub use responses::{ModelRequest, ModelResponse};
pub use tools::{FunctionDefinition, Tool, ToolCall, ToolChoice};
pub use usage::Usage;
