//! Tests for the responses module types.

mod common;

use pretty_assertions::assert_eq;
use serde_json::json;
use xai_openapi::responses::{
    DeleteStoredCompletionResponse, ModelInput, ModelInputContent, ModelInputContentItem,
    ModelInputItem, ModelInputPart, ModelRequest, ModelResponse, ModelResponseConfiguration,
    ModelResponseFormat, OutputMessage, Reasoning, ReasoningConfiguration, ReasoningText,
};
use xai_openapi::tools::FunctionToolCallOutput;

#[test]
fn test_model_request_minimal() {
    let json = json!({
        "input": "Hello!"
    });
    let request: ModelRequest = serde_json::from_value(json).unwrap();
    match request.input {
        ModelInput::Text(s) => assert_eq!(s, "Hello!"),
        _ => panic!("Expected text input"),
    }
}

#[test]
fn test_model_request_full() {
    let json = json!({
        "input": "What is 2+2?",
        "model": "grok-3",
        "instructions": "Be concise",
        "temperature": 0.5,
        "top_p": 0.9,
        "max_output_tokens": 500,
        "stream": false,
        "logprobs": true,
        "store": true
    });

    let request: ModelRequest = common::test_roundtrip(json);
    assert_eq!(request.model, Some("grok-3".to_string()));
    assert_eq!(request.instructions, Some("Be concise".to_string()));
    assert_eq!(request.temperature, Some(0.5));
    assert_eq!(request.max_output_tokens, Some(500));
}

#[test]
fn test_model_request_default_roundtrip() {
    common::test_default_roundtrip::<ModelRequest>();
}

#[test]
fn test_model_input_text() {
    let json = json!("Simple text input");
    let input: ModelInput = serde_json::from_value(json).unwrap();
    match input {
        ModelInput::Text(s) => assert_eq!(s, "Simple text input"),
        _ => panic!("Expected text"),
    }
}

#[test]
fn test_model_input_parts() {
    let json = json!([
        {
            "role": "user",
            "content": "Hello"
        }
    ]);

    let input: ModelInput = serde_json::from_value(json).unwrap();
    match input {
        ModelInput::Parts(parts) => {
            assert_eq!(parts.len(), 1);
        }
        _ => panic!("Expected parts"),
    }
}

#[test]
fn test_model_input_part_message() {
    let json = json!({
        "role": "user",
        "content": "What is AI?"
    });

    let part: ModelInputPart = serde_json::from_value(json).unwrap();
    match part {
        ModelInputPart::Message { role, content, .. } => {
            assert_eq!(role, "user");
            match content {
                ModelInputContent::Text(s) => assert_eq!(s, "What is AI?"),
                _ => panic!("Expected text content"),
            }
        }
        _ => panic!("Expected message"),
    }
}

#[test]
fn test_model_input_content_text() {
    let json = json!("Text content");
    let content: ModelInputContent = serde_json::from_value(json).unwrap();
    match content {
        ModelInputContent::Text(s) => assert_eq!(s, "Text content"),
        _ => panic!("Expected text"),
    }
}

#[test]
fn test_model_input_content_parts() {
    let json = json!([
        {
            "type": "input_text",
            "text": "Describe this image"
        },
        {
            "type": "input_image",
            "image_url": "https://example.com/img.png"
        }
    ]);

    let content: ModelInputContent = serde_json::from_value(json).unwrap();
    match content {
        ModelInputContent::Parts(parts) => {
            assert_eq!(parts.len(), 2);
        }
        _ => panic!("Expected parts"),
    }
}

#[test]
fn test_model_input_content_item_text() {
    let json = json!({
        "type": "input_text",
        "text": "Some text"
    });

    let item: ModelInputContentItem = common::test_roundtrip(json);
    match item {
        ModelInputContentItem::InputText { text } => assert_eq!(text, "Some text"),
        _ => panic!("Expected input_text"),
    }
}

#[test]
fn test_model_input_content_item_image() {
    let json = json!({
        "type": "input_image",
        "image_url": "https://example.com/img.jpg",
        "detail": "high"
    });

    let item: ModelInputContentItem = common::test_roundtrip(json);
    match item {
        ModelInputContentItem::InputImage {
            image_url, detail, ..
        } => {
            assert_eq!(image_url, "https://example.com/img.jpg");
            assert_eq!(detail, Some("high".to_string()));
        }
        _ => panic!("Expected input_image"),
    }
}

#[test]
fn test_model_input_content_item_file() {
    let json = json!({
        "type": "input_file",
        "file_id": "file_abc123"
    });

    let item: ModelInputContentItem = common::test_roundtrip(json);
    match item {
        ModelInputContentItem::InputFile { file_id } => assert_eq!(file_id, "file_abc123"),
        _ => panic!("Expected input_file"),
    }
}

#[test]
fn test_model_input_item_function_output() {
    let json = json!({
        "type": "function_call_output",
        "call_id": "call_123",
        "output": "{\"result\": 42}"
    });

    let item: ModelInputItem = serde_json::from_value(json).unwrap();
    match item {
        ModelInputItem::FunctionOutput(output) => {
            assert_eq!(output.call_id, "call_123");
            assert_eq!(output.output, "{\"result\": 42}");
        }
        _ => panic!("Expected function output"),
    }
}

#[test]
fn test_model_response() {
    let json = json!({
        "id": "resp_123",
        "object": "response",
        "created_at": 1700000000,
        "model": "grok-3",
        "output": [
            {
                "type": "message",
                "role": "assistant",
                "content": [
                    {
                        "type": "output_text",
                        "text": "Hello!",
                        "annotations": []
                    }
                ]
            }
        ],
        "parallel_tool_calls": false,
        "text": {},
        "tool_choice": "auto",
        "tools": [],
        "status": "completed",
        "store": false,
        "metadata": {}
    });

    let response: ModelResponse = common::test_roundtrip(json);
    assert_eq!(response.id, "resp_123");
    assert_eq!(response.object, "response");
    assert_eq!(response.status, "completed");
    assert_eq!(response.output.len(), 1);
}

#[test]
fn test_model_response_default_roundtrip() {
    common::test_default_roundtrip::<ModelResponse>();
}

#[test]
fn test_output_message() {
    let json = json!({
        "type": "message",
        "role": "assistant",
        "content": [
            {
                "type": "output_text",
                "text": "Response text",
                "annotations": []
            }
        ],
        "id": "msg_456",
        "status": "completed"
    });

    let message: OutputMessage = common::test_roundtrip(json);
    assert_eq!(message.message_type, "message");
    assert_eq!(message.role, "assistant");
    assert_eq!(message.id, Some("msg_456".to_string()));
    assert_eq!(message.status, Some("completed".to_string()));
}

#[test]
fn test_output_message_default_roundtrip() {
    common::test_default_roundtrip::<OutputMessage>();
}

#[test]
fn test_reasoning_configuration() {
    let json = json!({
        "effort": "high",
        "summary": "thinking summary"
    });

    let config: ReasoningConfiguration = common::test_roundtrip(json);
    assert_eq!(config.effort, Some("high".to_string()));
    assert_eq!(config.summary, Some("thinking summary".to_string()));
}

#[test]
fn test_reasoning_configuration_default_roundtrip() {
    common::test_default_roundtrip::<ReasoningConfiguration>();
}

#[test]
fn test_reasoning() {
    let json = json!({
        "type": "reasoning",
        "summary": [
            {
                "type": "summary_text",
                "text": "First I analyzed..."
            }
        ],
        "id": "reason_123",
        "status": "completed"
    });

    let reasoning: Reasoning = common::test_roundtrip(json);
    assert_eq!(reasoning.reasoning_type, "reasoning");
    assert_eq!(reasoning.summary.len(), 1);
    assert_eq!(reasoning.id, Some("reason_123".to_string()));
}

#[test]
fn test_reasoning_default_roundtrip() {
    common::test_default_roundtrip::<Reasoning>();
}

#[test]
fn test_reasoning_text() {
    let json = json!({
        "type": "summary_text",
        "text": "This is reasoning text"
    });

    let text: ReasoningText = common::test_roundtrip(json);
    assert_eq!(text.text_type, "summary_text");
    assert_eq!(text.text, "This is reasoning text");
}

#[test]
fn test_model_response_configuration() {
    let json = json!({
        "format": {
            "type": "json_object"
        }
    });

    let config: ModelResponseConfiguration = common::test_roundtrip(json);
    assert!(config.format.is_some());
}

#[test]
fn test_model_response_format_text() {
    let json = json!({"type": "text"});
    let format: ModelResponseFormat = serde_json::from_value(json).unwrap();
    assert_eq!(format, ModelResponseFormat::Text);
}

#[test]
fn test_model_response_format_json_object() {
    let json = json!({"type": "json_object"});
    let format: ModelResponseFormat = serde_json::from_value(json).unwrap();
    assert_eq!(format, ModelResponseFormat::JsonObject);
}

#[test]
fn test_model_response_format_json_schema() {
    let json = json!({
        "type": "json_schema",
        "schema": {
            "type": "object",
            "properties": {
                "name": {"type": "string"}
            }
        },
        "name": "PersonSchema"
    });

    let format: ModelResponseFormat = common::test_roundtrip(json);
    match format {
        ModelResponseFormat::JsonSchema { schema, name, .. } => {
            assert!(schema.is_object());
            assert_eq!(name, Some("PersonSchema".to_string()));
        }
        _ => panic!("Expected json_schema"),
    }
}

#[test]
fn test_delete_stored_completion_response() {
    let json = json!({
        "id": "resp_123",
        "object": "response",
        "deleted": true
    });

    let response: DeleteStoredCompletionResponse = common::test_roundtrip(json);
    assert_eq!(response.id, "resp_123");
    assert_eq!(response.object, "response");
    assert!(response.deleted);
}

#[test]
fn test_delete_stored_completion_response_default_roundtrip() {
    common::test_default_roundtrip::<DeleteStoredCompletionResponse>();
}

#[test]
fn test_function_tool_call_output() {
    let json = json!({
        "type": "function_call_output",
        "call_id": "call_abc",
        "output": "{\"temperature\": 72}"
    });

    let output: FunctionToolCallOutput = common::test_roundtrip(json);
    assert_eq!(output.output_type, "function_call_output");
    assert_eq!(output.call_id, "call_abc");
    assert_eq!(output.output, "{\"temperature\": 72}");
}
