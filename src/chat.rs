//! Chat completions API types for `/v1/chat/completions` endpoint.

use serde::{Deserialize, Serialize};

use crate::prelude::*;

use crate::common::{Content, DebugOutput, ResponseFormat, StreamOptions};
use crate::search::SearchParameters;
use crate::tools::{Tool, ToolCall, ToolChoice};
use crate::usage::Usage;

/// The chat request body for `/v1/chat/completions` endpoint.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ChatRequest {
    /// A list of messages that make up the chat conversation.
    #[serde(default)]
    pub messages: Vec<Message>,

    /// Model name for the model to use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,

    /// What sampling temperature to use, between 0 and 2.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,

    /// An alternative to sampling with temperature, called nucleus sampling.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,

    /// How many chat completion choices to generate for each input message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<i32>,

    /// If set, partial message deltas will be sent as server-sent events.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,

    /// Options for streaming response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_options: Option<StreamOptions>,

    /// (Not supported by reasoning models) Up to 4 sequences where the API will stop generating.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<Vec<String>>,

    /// An upper bound for the number of tokens that can be generated for a completion.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_completion_tokens: Option<i32>,

    /// \[DEPRECATED\] The maximum number of tokens that can be generated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<i32>,

    /// (Not supported by `grok-3` and reasoning models) Presence penalty.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<f32>,

    /// (Not supported by reasoning models) Frequency penalty.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<f32>,

    /// (Unsupported) A JSON object that maps tokens to an associated bias value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logit_bias: Option<serde_json::Value>,

    /// Whether to return log probabilities of the output tokens.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logprobs: Option<bool>,

    /// Number of most likely tokens to return at each token position.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_logprobs: Option<i32>,

    /// A unique identifier representing your end-user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,

    /// An object specifying the format that the model must output.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<ResponseFormat>,

    /// If specified, system will make a best effort to sample deterministically.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i32>,

    /// A list of tools the model may call.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<Tool>>,

    /// Controls which (if any) tool is called by the model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<ToolChoice>,

    /// If set to false, the model can perform maximum one tool call.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parallel_tool_calls: Option<bool>,

    /// Set the parameters to be used for searched data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_parameters: Option<SearchParameters>,

    /// Options to control the web search.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_search_options: Option<crate::tools::WebSearchOptions>,

    /// Constrains how hard a reasoning model thinks before responding.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning_effort: Option<String>,

    /// If set to `true`, the request returns a `request_id` for deferred completion.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deferred: Option<bool>,

    /// (Internal) Bootstrap host address for disaggregated prefill/decode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bootstrap_host: Option<String>,

    /// (Internal) Bootstrap room ID for disaggregated prefill/decode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bootstrap_room: Option<i64>,
}

/// Chat message objects.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "role", rename_all = "lowercase")]
pub enum Message {
    /// System message, usually instructions for the model to respond in a certain way.
    System {
        /// System prompt content.
        content: Content,
        /// A unique identifier representing your end-user.
        #[serde(skip_serializing_if = "Option::is_none")]
        name: Option<String>,
    },
    /// User message, typically request from user for the model to answer.
    User {
        /// User prompt content.
        content: Content,
        /// A unique identifier representing your end-user.
        #[serde(skip_serializing_if = "Option::is_none")]
        name: Option<String>,
    },
    /// Assistant role message, previous chat messages from the model.
    Assistant {
        /// Assistant prompt content.
        #[serde(skip_serializing_if = "Option::is_none")]
        content: Option<Content>,
        /// A unique identifier representing your end-user.
        #[serde(skip_serializing_if = "Option::is_none")]
        name: Option<String>,
        /// Assistant reasoning content.
        #[serde(skip_serializing_if = "Option::is_none")]
        reasoning_content: Option<String>,
        /// An array of tool calls available to the model.
        #[serde(skip_serializing_if = "Option::is_none")]
        tool_calls: Option<Vec<ToolCall>>,
    },
    /// Tool call role message, used to return function call result to the model.
    Tool {
        /// Content of the tool call result.
        content: Content,
        /// The ID of the tool call received from assistant message response.
        #[serde(skip_serializing_if = "Option::is_none")]
        tool_call_id: Option<String>,
    },
    /// Function call role message. Deprecated in favor of `tool`.
    Function {
        /// Content of the tool call result.
        content: Content,
        /// A unique identifier representing your end-user.
        #[serde(skip_serializing_if = "Option::is_none")]
        name: Option<String>,
    },
}

impl Default for Message {
    fn default() -> Self {
        Message::User {
            content: Content::default(),
            name: None,
        }
    }
}

/// The chat response body for `/v1/chat/completions` endpoint.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ChatResponse {
    /// A unique ID for the chat response.
    pub id: String,

    /// The object type, which is always `"chat.completion"`.
    pub object: String,

    /// The chat completion creation time in Unix timestamp.
    pub created: i64,

    /// Model ID used to create chat completion.
    pub model: String,

    /// A list of response choices from the model.
    pub choices: Vec<Choice>,

    /// Token usage information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<Usage>,

    /// System fingerprint, used to indicate xAI system configuration changes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_fingerprint: Option<String>,

    /// List of all the external pages used by the model to answer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub citations: Option<Vec<String>>,

    /// Debug output. Only available to trusted testers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub debug_output: Option<DebugOutput>,
}

/// A response choice from the model.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Choice {
    /// Index of the choice within the response choices, starting from 0.
    pub index: i32,

    /// The generated chat completion message.
    pub message: ChoiceMessage,

    /// Finish reason.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finish_reason: Option<String>,

    /// The log probabilities of each output token.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logprobs: Option<LogProbs>,
}

/// The generated chat completion message.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ChoiceMessage {
    /// The role that the message belongs to, the response from model is always `"assistant"`.
    pub role: String,

    /// The content of the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,

    /// The reasoning trace generated by the model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning_content: Option<String>,

    /// The reason given by model if unable to generate a response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refusal: Option<String>,

    /// A list of tool calls asked by model for user to perform.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<ToolCall>>,
}

/// Streaming chat response chunk.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ChatResponseChunk {
    /// A unique ID for the chat response chunk.
    pub id: String,

    /// The object type, which is always `"chat.completion.chunk"`.
    pub object: String,

    /// The chat completion creation time in Unix timestamp.
    pub created: i64,

    /// The model ID used to create chat completion.
    pub model: String,

    /// A list of response choices from the model.
    pub choices: Vec<ChoiceChunk>,

    /// Token usage information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<Usage>,

    /// System fingerprint, used to indicate xAI system configuration changes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_fingerprint: Option<String>,

    /// List of all the external pages used by the model to answer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub citations: Option<Vec<String>>,

    /// Debug output. Only available to trusted testers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub debug_output: Option<DebugOutput>,
}

/// A streaming response choice chunk.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ChoiceChunk {
    /// Index of the choice.
    pub index: i32,

    /// Additional difference (delta) of the result.
    pub delta: Delta,

    /// Finish reason.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finish_reason: Option<String>,

    /// The log probabilities of each output token.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logprobs: Option<LogProbs>,
}

/// Delta content in streaming response.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Delta {
    /// The role of the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,

    /// The content delta.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,

    /// The reasoning content delta.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning_content: Option<String>,

    /// Tool calls delta.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<ToolCall>>,

    /// Images generated by the model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<Vec<String>>,
}

/// Log probabilities.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct LogProbs {
    /// An array of the log probabilities of each output token returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<Vec<TokenLogProb>>,
}

/// Token log probability.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct TokenLogProb {
    /// The token.
    pub token: String,

    /// The log probability of returning this token.
    pub logprob: f32,

    /// An array of the most likely tokens to return at this token position.
    pub top_logprobs: Vec<TopLogProb>,

    /// The ASCII encoding of the output character.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes: Option<Vec<u32>>,
}

/// Top log probability.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct TopLogProb {
    /// The token.
    pub token: String,

    /// The log probability of returning this token.
    pub logprob: f32,

    /// The ASCII encoding of the output character.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes: Option<Vec<u32>>,
}
