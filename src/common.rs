//! Common types shared across multiple xAI API endpoints.

use serde::{Deserialize, Serialize};

use crate::prelude::*;

/// Content of each chat message.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Content {
    /// Text prompt.
    Text(String),
    /// An array of content parts of different types, such as image, text or text file.
    Parts(Vec<ContentPart>),
}

impl Default for Content {
    fn default() -> Self {
        Content::Text(String::new())
    }
}

impl From<&str> for Content {
    fn from(s: &str) -> Self {
        Content::Text(s.to_string())
    }
}

impl From<String> for Content {
    fn from(s: String) -> Self {
        Content::Text(s)
    }
}

/// A part of content in a message.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ContentPart {
    /// The type of the content part.
    #[serde(rename = "type")]
    pub content_type: String,

    /// Text prompt.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,

    /// A public URL of image prompt, only available for vision models.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<ImageUrl>,

    /// Specifies the detail level of the image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,

    /// File path to a text file to be used as prompt.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_file: Option<String>,

    /// File reference for file attachments (OpenAI-compatible nesting).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<FileRef>,
}

/// Image URL object of image prompt.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ImageUrl {
    /// URL of the image.
    pub url: String,

    /// Specifies the detail level of the image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
}

/// File reference for file attachments.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct FileRef {
    /// The file ID from the Files API.
    pub file_id: String,
}

/// Options available when using streaming response.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct StreamOptions {
    /// Set an additional chunk to be streamed before the `data: [DONE]` message.
    /// The other chunks will return `null` in `usage` field.
    pub include_usage: bool,
}

/// Response format parameter for structured outputs.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ResponseFormat {
    /// Specify text response format, always `"text"`.
    #[default]
    Text,
    /// Specify `json_object` response format, always `json_object`.
    /// Used for backward compatibility. Prefer to use `"json_schema"` instead.
    JsonObject,
    /// Specify `json_schema` response format with a given schema.
    JsonSchema {
        /// A json schema representing the desired response schema.
        json_schema: serde_json::Value,
    },
}

/// Annotation on text output.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Annotation {
    /// The type of the annotation. Only supported type currently is `url_citation`.
    #[serde(rename = "type")]
    pub annotation_type: String,

    /// The URL of the web resource.
    pub url: String,

    /// The title of the annotation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    /// The start index of the annotation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_index: Option<i32>,

    /// The end index of the annotation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_index: Option<i32>,
}

/// Details about why a response is incomplete.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct IncompleteDetails {
    /// The reason why the response is incomplete.
    pub reason: String,
}

/// API key information.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ApiKey {
    /// The redacted API key.
    pub redacted_api_key: String,

    /// User ID the API key belongs to.
    pub user_id: String,

    /// The name of the API key specified by user.
    pub name: String,

    /// Creation time of the API key in Unix timestamp.
    pub create_time: String,

    /// Last modification time of the API key in Unix timestamp.
    pub modify_time: String,

    /// User ID of the user who last modified the API key.
    pub modified_by: String,

    /// The team ID of the team that owns the API key.
    pub team_id: String,

    /// A list of ACLs authorized with the API key.
    pub acls: Vec<String>,

    /// ID of the API key.
    pub api_key_id: String,

    /// Indicates whether the team that owns the API key is blocked.
    pub team_blocked: bool,

    /// Indicates whether the API key is blocked.
    pub api_key_blocked: bool,

    /// Indicates whether the API key is disabled.
    pub api_key_disabled: bool,
}

/// (Legacy) Anthropic compatible complete request on `/v1/complete` endpoint.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct CompleteRequest {
    /// Model to use for completion.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,

    /// Prompt for the model to perform completion on.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,

    /// The maximum number of tokens to generate before stopping.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens_to_sample: Option<i32>,

    /// What sampling temperature to use, between 0 and 2.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,

    /// An alternative to sampling with temperature, called nucleus sampling.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,

    /// (Unsupported) When generating next tokens, randomly selecting from the k most likely options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_k: Option<i32>,

    /// (Not supported by reasoning models) Up to 4 sequences where the API will stop generating.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_sequences: Option<Vec<String>>,

    /// (Unsupported) If set, partial message deltas will be sent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,

    /// An object describing metadata about the request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<MessageMetadata>,
}

/// (Legacy) Anthropic compatible complete response on `/v1/complete` endpoint.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct CompleteResponse {
    /// Completion response object type. This is always `"completion"`.
    #[serde(rename = "type")]
    pub response_type: String,

    /// ID of the completion response.
    pub id: String,

    /// The completion content up to and excluding stop sequences.
    pub completion: String,

    /// The model that handled the request.
    pub model: String,

    /// The reason to stop completion.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_reason: Option<String>,
}

/// Message metadata.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct MessageMetadata {
    /// A unique identifier representing your end-user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

/// (Legacy) Request for `/v1/completions` endpoint.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SampleRequest {
    /// Specifies the model to be used for the request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,

    /// Input for generating completions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<SampleContent>,

    /// Limits the number of tokens that can be produced in the output.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<i32>,

    /// What sampling temperature to use, between 0 and 2.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,

    /// An alternative to sampling with temperature, called nucleus sampling.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,

    /// Determines how many completion sequences to produce for each prompt.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<i32>,

    /// Whether to stream back partial progress.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,

    /// Options for streaming response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_options: Option<StreamOptions>,

    /// (Not supported by reasoning models) Up to 4 sequences where the API will stop generating.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<Vec<String>>,

    /// (Not supported by `grok-3` and reasoning models) Presence penalty.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<f32>,

    /// (Unsupported) Frequency penalty.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<f32>,

    /// Option to include the original prompt in the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub echo: Option<bool>,

    /// Include the log probabilities on the most likely output tokens.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logprobs: Option<bool>,

    /// (Unsupported) Generates multiple completions internally.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub best_of: Option<i32>,

    /// (Unsupported) Logit bias.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logit_bias: Option<serde_json::Value>,

    /// If specified, system will make a best effort to sample deterministically.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i32>,

    /// (Unsupported) Optional string to append after the generated text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suffix: Option<String>,

    /// A unique identifier representing your end-user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

/// Sample content input.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SampleContent {
    /// Text prompt.
    Text(String),
    /// An array of strings, a token list, or an array of token lists.
    Array(Vec<String>),
}

impl Default for SampleContent {
    fn default() -> Self {
        SampleContent::Text(String::new())
    }
}

/// (Legacy) Response for `/v1/completions` endpoint.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SampleResponse {
    /// ID of the request.
    pub id: String,

    /// Object type of the response. This is always `"text_completion"`.
    pub object: String,

    /// The chat completion creation time in Unix timestamp.
    pub created: i64,

    /// Model used.
    pub model: String,

    /// A list of response choices from the model.
    pub choices: Vec<SampleChoice>,

    /// System fingerprint, used to indicate xAI system configuration changes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_fingerprint: Option<String>,

    /// Token usage information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<crate::usage::Usage>,
}

/// A choice in a sample response.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SampleChoice {
    /// Index of the choice.
    pub index: i32,

    /// Text response.
    pub text: String,

    /// Finish reason.
    pub finish_reason: String,
}

/// A unique request ID for deferred chat response.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct StartDeferredChatResponse {
    /// A unique request ID for the chat response.
    pub request_id: String,
}

/// `DocumentsSource` defines the source of documents to search over.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct DocumentsSource {
    /// The collection IDs to search in.
    pub collection_ids: Vec<String>,
}

/// Deprecated: Metric now comes from collection creation.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub enum RankingMetric {
    #[default]
    #[serde(rename = "RANKING_METRIC_UNKNOWN")]
    Unknown,
    #[serde(rename = "RANKING_METRIC_L2_DISTANCE")]
    L2Distance,
    #[serde(rename = "RANKING_METRIC_COSINE_SIMILARITY")]
    CosineSimilarity,
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

/// A part of system message content.
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

/// Debug output. Only available to trusted testers.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct DebugOutput {
    /// Number of attempts made to the model.
    pub attempts: i32,

    /// The request received from the user.
    pub request: String,

    /// The prompt sent to the model in text form.
    pub prompt: String,

    /// JSON-serialized request sent to the inference engine.
    pub engine_request: String,

    /// The response(s) received from the model.
    pub responses: Vec<String>,

    /// The individual chunks returned from the pipeline of samplers.
    pub chunks: Vec<String>,

    /// Number of cache reads.
    pub cache_read_count: i32,

    /// Size of cache read.
    pub cache_read_input_bytes: i64,

    /// Number of cache writes.
    pub cache_write_count: i32,

    /// Size of cache write.
    pub cache_write_input_bytes: i64,

    /// The load balancer address.
    pub lb_address: String,

    /// The tag of the actual engines sitting behind the GTP address.
    pub sampler_tag: String,

    /// The underlying checkpoint mount path for the sampler that served this request.
    pub sampler_checkpoint_mount: String,
}
