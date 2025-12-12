//! Responses API types for `/v1/responses` endpoint.

use serde::{Deserialize, Serialize};

use crate::prelude::*;

use crate::common::{DebugOutput, IncompleteDetails};
use crate::search::SearchParameters;
use crate::tools::{
    CodeInterpreterCall, CustomToolCall, FileSearchCall, FunctionToolCall, FunctionToolCallOutput,
    McpCall, ModelTool, ModelToolChoice, OutputMessageContent, WebSearchCall,
};
use crate::usage::ModelUsage;

/// The request body for `/v1/responses` endpoint.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ModelRequest {
    /// The input passed to the model. Can be text, image or file.
    pub input: ModelInput,

    /// Model name for the model to use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,

    /// An alternate way to specify the system prompt.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,

    /// What sampling temperature to use, between 0 and 2.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,

    /// An alternative to sampling with temperature, called nucleus sampling.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,

    /// Max number of tokens that can be generated in a response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_output_tokens: Option<i32>,

    /// If set, partial message deltas will be sent as server-sent events.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,

    /// Whether to return log probabilities of the output tokens.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logprobs: Option<bool>,

    /// Whether to store the input message(s) and model response for later retrieval.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub store: Option<bool>,

    /// The ID of the previous response from the model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_response_id: Option<String>,

    /// A list of tools the model may call.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<ModelTool>>,

    /// Controls which (if any) tool is called by the model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<ModelToolChoice>,

    /// Whether to allow the model to run parallel tool calls.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parallel_tool_calls: Option<bool>,

    /// Set the parameters to be used for searched data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_parameters: Option<SearchParameters>,

    /// Reasoning configuration. Only for reasoning models.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning: Option<ReasoningConfiguration>,

    /// Settings for customizing a text response from the model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<ModelResponseConfiguration>,

    /// What additional output data to include in the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include: Option<Vec<String>>,

    /// Not supported. Only maintained for compatibility reasons.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,

    /// Not supported. Only maintained for compatibility reasons.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_tier: Option<String>,

    /// (Unsupported) Whether to process the response asynchronously in the background.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background: Option<bool>,
}

/// Content of the input passed to a `/v1/responses` request.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ModelInput {
    /// Text input.
    Text(String),
    /// A list of input items to the model.
    Parts(Vec<ModelInputPart>),
}

impl Default for ModelInput {
    fn default() -> Self {
        ModelInput::Text(String::new())
    }
}

/// A part of model input.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ModelInputPart {
    /// Message input to the model.
    Message {
        /// The role of the message.
        role: String,
        /// Text, image or audio input.
        content: ModelInputContent,
        /// A unique identifier representing your end-user.
        #[serde(skip_serializing_if = "Option::is_none")]
        name: Option<String>,
        /// The type of the message, which is always `message`.
        #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
        message_type: Option<String>,
    },
    /// Previous responses of the model and tool call outputs.
    Item(ModelInputItem),
}

impl Default for ModelInputPart {
    fn default() -> Self {
        ModelInputPart::Message {
            role: "user".to_string(),
            content: ModelInputContent::default(),
            name: None,
            message_type: None,
        }
    }
}

/// Model input content.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ModelInputContent {
    /// Text input.
    Text(String),
    /// A list of input items to the model.
    Parts(Vec<ModelInputContentItem>),
}

impl Default for ModelInputContent {
    fn default() -> Self {
        ModelInputContent::Text(String::new())
    }
}

/// A model input content item.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ModelInputContentItem {
    /// Text input.
    InputText {
        /// Text input.
        text: String,
    },
    /// Image input.
    InputImage {
        /// A public URL of image prompt, only available for vision models.
        image_url: String,
        /// Specifies the detail level of the image.
        #[serde(skip_serializing_if = "Option::is_none")]
        detail: Option<String>,
        /// Only included for compatibility.
        #[serde(skip_serializing_if = "Option::is_none")]
        file_id: Option<String>,
    },
    /// File input.
    InputFile {
        /// The file ID from the Files API.
        file_id: String,
    },
}

impl Default for ModelInputContentItem {
    fn default() -> Self {
        ModelInputContentItem::InputText {
            text: String::new(),
        }
    }
}

/// Model input item (previous outputs or tool call outputs).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ModelInputItem {
    /// Previous model output.
    Output(ModelOutput),
    /// Function tool call output.
    FunctionOutput(FunctionToolCallOutput),
}

impl Default for ModelInputItem {
    fn default() -> Self {
        ModelInputItem::FunctionOutput(FunctionToolCallOutput::default())
    }
}

/// The response body for `/v1/responses` endpoint.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ModelResponse {
    /// Unique ID of the response.
    pub id: String,

    /// The object type of this resource. Always set to `response`.
    pub object: String,

    /// The Unix timestamp (in seconds) for the response creation time.
    pub created_at: i64,

    /// Model name used to generate the response.
    pub model: String,

    /// The response generated by the model.
    pub output: Vec<ModelOutput>,

    /// Whether to allow the model to run parallel tool calls.
    pub parallel_tool_calls: bool,

    /// Settings for customizing a text response from the model.
    pub text: ModelResponseConfiguration,

    /// Controls which (if any) tool is called by the model.
    pub tool_choice: ModelToolChoice,

    /// A list of tools the model may call.
    pub tools: Vec<ModelTool>,

    /// Status of the response. One of `completed`, `in_progress` or `incomplete`.
    pub status: String,

    /// Whether to store the input message(s) and model response for later retrieval.
    pub store: bool,

    /// Only included for compatibility.
    pub metadata: serde_json::Value,

    /// Token usage information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<ModelUsage>,

    /// What sampling temperature to use, between 0 and 2.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,

    /// An alternative to sampling with temperature, called nucleus sampling.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,

    /// Max number of tokens that can be generated in a response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_output_tokens: Option<i32>,

    /// The ID of the previous response from the model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_response_id: Option<String>,

    /// Reasoning configuration. Only for reasoning models.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning: Option<ReasoningConfiguration>,

    /// A unique identifier representing your end-user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,

    /// Details about why the response is incomplete.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub incomplete_details: Option<IncompleteDetails>,

    /// Whether to process the response asynchronously in the background.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background: Option<bool>,

    /// Debug output. Only available to trusted testers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub debug_output: Option<DebugOutput>,
}

/// Model output.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ModelOutput {
    /// An output message from the model.
    Message(OutputMessage),
    /// A function tool call.
    FunctionCall(FunctionToolCall),
    /// Reasoning done by the model.
    Reasoning(Reasoning),
    /// A web search tool call.
    WebSearch(WebSearchCall),
    /// A file search tool call.
    FileSearch(FileSearchCall),
    /// A code interpreter tool call.
    CodeInterpreter(CodeInterpreterCall),
    /// A MCP tool call.
    Mcp(McpCall),
    /// A custom tool call.
    Custom(CustomToolCall),
}

impl Default for ModelOutput {
    fn default() -> Self {
        ModelOutput::Message(OutputMessage::default())
    }
}

/// An output message from the model.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct OutputMessage {
    /// The type of the output message, which is always `message`.
    #[serde(rename = "type")]
    pub message_type: String,

    /// The role of the output message, which can be `assistant` or `tool`.
    pub role: String,

    /// Content of the output message.
    pub content: Vec<OutputMessageContent>,

    /// The unique ID of the output message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Status of the item. One of `completed`, `in_progress` or `incomplete`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// Reasoning configuration.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ReasoningConfiguration {
    /// Constrains how hard a reasoning model thinks before responding.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effort: Option<String>,

    /// A summary of the model's reasoning process.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,

    /// Only included for compatibility.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generate_summary: Option<String>,
}

/// The reasoning done by the model.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Reasoning {
    /// The type of the object, which is always `reasoning`.
    #[serde(rename = "type")]
    pub reasoning_type: String,

    /// The reasoning text contents.
    pub summary: Vec<ReasoningText>,

    /// The unique ID of the reasoning content.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The encrypted reasoning.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encrypted_content: Option<String>,

    /// Status of the item. One of `completed`, `in_progress` or `incomplete`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// Reasoning text.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ReasoningText {
    /// The type of the object, which is always `summary_text`.
    #[serde(rename = "type")]
    pub text_type: String,

    /// Reasoning done by the model.
    pub text: String,
}

/// Settings for customizing a text response from the model.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ModelResponseConfiguration {
    /// An object specifying the format that the model must output.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<ModelResponseFormat>,
}

/// Response format parameter for structured outputs (Responses API).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ModelResponseFormat {
    /// Specify text response format, always `"text"`.
    #[default]
    Text,
    /// Specify json_object response format.
    JsonObject,
    /// Specify json_schema response format with a given schema.
    JsonSchema {
        /// A json schema representing the desired response schema.
        schema: serde_json::Value,
        /// Only included for compatibility.
        #[serde(skip_serializing_if = "Option::is_none")]
        name: Option<String>,
        /// Only included for compatibility.
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        /// Only included for compatibility.
        #[serde(skip_serializing_if = "Option::is_none")]
        strict: Option<bool>,
    },
}

/// Response to delete a stored completion.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct DeleteStoredCompletionResponse {
    /// The response_id to be deleted.
    pub id: String,

    /// The deleted object type, which is always `response`.
    pub object: String,

    /// Whether the response was successfully deleted.
    pub deleted: bool,
}
