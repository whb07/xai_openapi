//! Anthropic-compatible messages API types for `/v1/messages` endpoint.

use serde::{Deserialize, Serialize};

use crate::prelude::*;

/// Request message for `/v1/messages` endpoint.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct MessageRequest {
    /// Model name for the model to use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,

    /// Input messages.
    #[serde(default)]
    pub messages: Vec<MessageBody>,

    /// The maximum number of tokens to generate before stopping.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<i32>,

    /// System prompt message for the model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<SystemMessageContent>,

    /// What sampling temperature to use, between 0 and 2.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,

    /// An alternative to sampling with temperature, called nucleus sampling.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,

    /// (Unsupported) When generating next tokens, randomly selecting from k most likely options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_k: Option<i32>,

    /// If set, partial message deltas will be sent as server-sent events.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,

    /// (Not supported by reasoning models) Up to 4 sequences where the API will stop generating.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_sequences: Option<Vec<String>>,

    /// A list of tools the model may call.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<MessageTools>>,

    /// Controls which (if any) tool is called by the model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<MessageToolChoice>,

    /// An object describing metadata about the request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<MessageMetadata>,
}

/// Anthropic compatible message body.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct MessageBody {
    /// The role that the message belongs to: `"system"`, `"user"`, or `"assistant"`.
    pub role: String,

    /// The content message.
    pub content: MessageContent,
}

/// Message content.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MessageContent {
    /// Text prompt.
    Text(String),
    /// An array of message content parts.
    Parts(Vec<MessageContentPart>),
}

impl Default for MessageContent {
    fn default() -> Self {
        MessageContent::Text(String::new())
    }
}

/// Message content part.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum MessageContentPart {
    /// Text prompt message content part.
    Text {
        /// Text prompt.
        text: String,
        /// (Unsupported) Cache control.
        #[serde(skip_serializing_if = "Option::is_none")]
        cache_control: Option<serde_json::Value>,
    },
    /// Image prompt message content part.
    Image {
        /// Image source.
        source: MessageImageContent,
        /// (Unsupported) Cache control.
        #[serde(skip_serializing_if = "Option::is_none")]
        cache_control: Option<serde_json::Value>,
    },
    /// Tool call message content part (received from model).
    ToolUse {
        /// ID of the tool call.
        id: String,
        /// Name of the tool call.
        name: String,
        /// Input for tool call.
        input: serde_json::Value,
        /// (Unsupported) Cache control.
        #[serde(skip_serializing_if = "Option::is_none")]
        cache_control: Option<serde_json::Value>,
    },
    /// Tool call result.
    ToolResult {
        /// ID of the tool call given by the model.
        tool_use_id: String,
        /// Result content of the tool call.
        content: String,
        /// Whether the tool call returns an error.
        #[serde(skip_serializing_if = "Option::is_none")]
        is_error: Option<bool>,
        /// (Unsupported) Cache control.
        #[serde(skip_serializing_if = "Option::is_none")]
        cache_control: Option<serde_json::Value>,
    },
    /// (Redacted) Thinking of the model.
    RedactedThinking {
        /// Encrypted data of the redacted thinking.
        data: String,
    },
    /// Thinking of the model.
    Thinking {
        /// Thinking.
        thinking: String,
    },
}

impl Default for MessageContentPart {
    fn default() -> Self {
        MessageContentPart::Text {
            text: String::new(),
            cache_control: None,
        }
    }
}

/// Message image content source.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum MessageImageContent {
    /// Base64 encoded image.
    Base64 {
        /// Base64 encoded image string.
        data: String,
        /// Media type of the image source: `image/jpeg`, `image/png`, `image/webp`.
        media_type: String,
    },
    /// URL of the image.
    Url {
        /// URL of the image.
        url: String,
    },
}

impl Default for MessageImageContent {
    fn default() -> Self {
        MessageImageContent::Url { url: String::new() }
    }
}

/// System message content.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SystemMessageContent {
    /// Text content of system prompt.
    Text(String),
    /// An array of system prompt parts.
    Parts(Vec<SystemMessagePart>),
}

impl Default for SystemMessageContent {
    fn default() -> Self {
        SystemMessageContent::Text(String::new())
    }
}

/// System message part.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SystemMessagePart {
    /// Type of the object. This is always `"text"`.
    #[serde(rename = "type")]
    pub part_type: String,

    /// System prompt text.
    pub text: String,

    /// (Unsupported) Cache control.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_control: Option<serde_json::Value>,
}

/// Response message for `/v1/messages` endpoint.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct MessageResponse {
    /// Unique object identifier.
    pub id: String,

    /// Object type. This is always `"message"` for message types.
    #[serde(rename = "type")]
    pub response_type: String,

    /// Role of the generated message. Always `"assistant"`.
    pub role: String,

    /// Response message content.
    pub content: Vec<MessageResponseContent>,

    /// Model name that handled the request.
    pub model: String,

    /// Token usage information.
    pub usage: MessageUsage,

    /// Reason to stop: `"stop_sequence"`, `"max_tokens"`, `"end_turn"`, or `"tool_use"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_reason: Option<String>,

    /// Custom stop sequence used to stop the generation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_sequence: Option<String>,
}

/// Message response content.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum MessageResponseContent {
    /// Text response from the model.
    Text {
        /// Text content.
        text: String,
    },
    /// Thinking response from the model.
    Thinking {
        /// Thinking content.
        thinking: String,
        /// Signature of the content.
        signature: String,
    },
    /// Redacted thinking response from the model.
    RedactedThinking {
        /// Signature of the content.
        data: String,
    },
    /// Request by the model to invoke a tool call.
    ToolUse {
        /// Tool call ID.
        id: String,
        /// Name of the tool call to be used.
        name: String,
        /// Input to the tool call following the `input_schema`.
        input: serde_json::Value,
    },
}

impl Default for MessageResponseContent {
    fn default() -> Self {
        MessageResponseContent::Text {
            text: String::new(),
        }
    }
}

/// Tool choice option for messages API.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum MessageToolChoice {
    /// Allows the model to automatically decide whether to call the tool.
    #[default]
    Auto,
    /// Forces the model to use at least one tool, without specifying the tool.
    Any,
    /// Forces the model to use the named tool.
    Tool {
        /// Name of the tool to use.
        name: String,
    },
}

/// Tool definition for messages API.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct MessageTools {
    /// Name of the tool.
    pub name: String,

    /// Description of the tool.
    pub description: String,

    /// Input schema allowed by the tool.
    pub input_schema: MessageToolInputSchema,

    /// (Unsupported) Cache control.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_control: Option<serde_json::Value>,
}

/// Tool input schema for messages API.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct MessageToolInputSchema {
    /// Type of the schema. This is always `"object"`.
    #[serde(rename = "type")]
    pub schema_type: String,

    /// JSON-object of the tool input schema.
    pub properties: serde_json::Value,

    /// Required properties of the tool input schema, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<Vec<String>>,
}

/// Metadata for messages API request.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct MessageMetadata {
    /// A unique identifier representing your end-user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

/// Token usage information for messages API.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct MessageUsage {
    /// Number of input tokens used.
    pub input_tokens: i32,

    /// Number of output tokens used.
    pub output_tokens: i32,

    /// Number of tokens retrieved from the cache for this request.
    pub cache_read_input_tokens: i32,

    /// (Unsupported) Number of tokens written to the cache when creating a new entry.
    pub cache_creation_input_tokens: i32,
}
