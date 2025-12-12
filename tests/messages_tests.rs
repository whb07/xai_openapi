//! Tests for the messages module types (Anthropic-compatible API).

mod common;

use pretty_assertions::assert_eq;
use serde_json::json;
use xai_openapi::messages::{
    MessageBody, MessageContent, MessageContentPart, MessageImageContent, MessageMetadata,
    MessageRequest, MessageResponse, MessageResponseContent, MessageToolChoice,
    MessageToolInputSchema, MessageTools, MessageUsage, SystemMessageContent, SystemMessagePart,
};

#[test]
fn test_message_request_minimal() {
    let json = json!({
        "messages": []
    });
    let request: MessageRequest = serde_json::from_value(json).unwrap();
    assert!(request.messages.is_empty());
    assert!(request.model.is_none());
}

#[test]
fn test_message_request_full() {
    let json = json!({
        "model": "grok-3",
        "messages": [
            {
                "role": "user",
                "content": "Hello!"
            }
        ],
        "max_tokens": 1000,
        "system": "You are a helpful assistant.",
        "temperature": 0.7,
        "top_p": 0.9,
        "stream": false
    });

    let request: MessageRequest = common::test_roundtrip(json);
    assert_eq!(request.model, Some("grok-3".to_string()));
    assert_eq!(request.max_tokens, Some(1000));
    assert_eq!(request.temperature, Some(0.7));
}

#[test]
fn test_message_request_default_roundtrip() {
    common::test_default_roundtrip::<MessageRequest>();
}

#[test]
fn test_message_body() {
    let json = json!({
        "role": "user",
        "content": "What is the weather?"
    });

    let body: MessageBody = common::test_roundtrip(json);
    assert_eq!(body.role, "user");
    match body.content {
        MessageContent::Text(s) => assert_eq!(s, "What is the weather?"),
        _ => panic!("Expected text content"),
    }
}

#[test]
fn test_message_content_text() {
    let json = json!("Hello, world!");
    let content: MessageContent = serde_json::from_value(json).unwrap();
    match content {
        MessageContent::Text(s) => assert_eq!(s, "Hello, world!"),
        _ => panic!("Expected text content"),
    }
}

#[test]
fn test_message_content_parts() {
    let json = json!([
        {
            "type": "text",
            "text": "What is in this image?"
        },
        {
            "type": "image",
            "source": {
                "type": "url",
                "url": "https://example.com/image.png"
            }
        }
    ]);

    let content: MessageContent = serde_json::from_value(json).unwrap();
    match content {
        MessageContent::Parts(parts) => {
            assert_eq!(parts.len(), 2);
        }
        _ => panic!("Expected parts content"),
    }
}

#[test]
fn test_message_content_part_text() {
    let json = json!({
        "type": "text",
        "text": "Hello!"
    });

    let part: MessageContentPart = common::test_roundtrip(json);
    match part {
        MessageContentPart::Text { text, .. } => assert_eq!(text, "Hello!"),
        _ => panic!("Expected text part"),
    }
}

#[test]
fn test_message_content_part_image() {
    let json = json!({
        "type": "image",
        "source": {
            "type": "base64",
            "data": "iVBORw0KGgo=",
            "media_type": "image/png"
        }
    });

    let part: MessageContentPart = common::test_roundtrip(json);
    match part {
        MessageContentPart::Image { source, .. } => match source {
            MessageImageContent::Base64 { data, media_type } => {
                assert_eq!(data, "iVBORw0KGgo=");
                assert_eq!(media_type, "image/png");
            }
            _ => panic!("Expected base64 image"),
        },
        _ => panic!("Expected image part"),
    }
}

#[test]
fn test_message_content_part_tool_use() {
    let json = json!({
        "type": "tool_use",
        "id": "tool_123",
        "name": "get_weather",
        "input": {"location": "SF"}
    });

    let part: MessageContentPart = common::test_roundtrip(json);
    match part {
        MessageContentPart::ToolUse {
            id, name, input, ..
        } => {
            assert_eq!(id, "tool_123");
            assert_eq!(name, "get_weather");
            assert_eq!(input["location"], "SF");
        }
        _ => panic!("Expected tool_use part"),
    }
}

#[test]
fn test_message_content_part_tool_result() {
    let json = json!({
        "type": "tool_result",
        "tool_use_id": "tool_123",
        "content": "72F and sunny",
        "is_error": false
    });

    let part: MessageContentPart = common::test_roundtrip(json);
    match part {
        MessageContentPart::ToolResult {
            tool_use_id,
            content,
            is_error,
            ..
        } => {
            assert_eq!(tool_use_id, "tool_123");
            assert_eq!(content, "72F and sunny");
            assert_eq!(is_error, Some(false));
        }
        _ => panic!("Expected tool_result part"),
    }
}

#[test]
fn test_message_content_part_thinking() {
    let json = json!({
        "type": "thinking",
        "thinking": "Let me think about this..."
    });

    let part: MessageContentPart = common::test_roundtrip(json);
    match part {
        MessageContentPart::Thinking { thinking } => {
            assert_eq!(thinking, "Let me think about this...");
        }
        _ => panic!("Expected thinking part"),
    }
}

#[test]
fn test_message_image_content_base64() {
    let json = json!({
        "type": "base64",
        "data": "SGVsbG8=",
        "media_type": "image/jpeg"
    });

    let content: MessageImageContent = common::test_roundtrip(json);
    match content {
        MessageImageContent::Base64 { data, media_type } => {
            assert_eq!(data, "SGVsbG8=");
            assert_eq!(media_type, "image/jpeg");
        }
        _ => panic!("Expected base64"),
    }
}

#[test]
fn test_message_image_content_url() {
    let json = json!({
        "type": "url",
        "url": "https://example.com/img.png"
    });

    let content: MessageImageContent = common::test_roundtrip(json);
    match content {
        MessageImageContent::Url { url } => {
            assert_eq!(url, "https://example.com/img.png");
        }
        _ => panic!("Expected url"),
    }
}

#[test]
fn test_system_message_content_text() {
    let json = json!("You are a helpful assistant.");
    let content: SystemMessageContent = serde_json::from_value(json).unwrap();
    match content {
        SystemMessageContent::Text(s) => assert_eq!(s, "You are a helpful assistant."),
        _ => panic!("Expected text"),
    }
}

#[test]
fn test_system_message_content_parts() {
    let json = json!([
        {
            "type": "text",
            "text": "Part 1"
        },
        {
            "type": "text",
            "text": "Part 2"
        }
    ]);

    let content: SystemMessageContent = serde_json::from_value(json).unwrap();
    match content {
        SystemMessageContent::Parts(parts) => {
            assert_eq!(parts.len(), 2);
        }
        _ => panic!("Expected parts"),
    }
}

#[test]
fn test_system_message_part() {
    let json = json!({
        "type": "text",
        "text": "System prompt text"
    });

    let part: SystemMessagePart = common::test_roundtrip(json);
    assert_eq!(part.part_type, "text");
    assert_eq!(part.text, "System prompt text");
}

#[test]
fn test_message_response() {
    let json = json!({
        "id": "msg_123",
        "type": "message",
        "role": "assistant",
        "content": [
            {
                "type": "text",
                "text": "Hello! How can I help?"
            }
        ],
        "model": "grok-3",
        "usage": {
            "input_tokens": 10,
            "output_tokens": 8,
            "cache_read_input_tokens": 0,
            "cache_creation_input_tokens": 0
        },
        "stop_reason": "end_turn"
    });

    let response: MessageResponse = common::test_roundtrip(json);
    assert_eq!(response.id, "msg_123");
    assert_eq!(response.response_type, "message");
    assert_eq!(response.role, "assistant");
    assert_eq!(response.content.len(), 1);
    assert_eq!(response.stop_reason, Some("end_turn".to_string()));
}

#[test]
fn test_message_response_default_roundtrip() {
    common::test_default_roundtrip::<MessageResponse>();
}

#[test]
fn test_message_response_content_text() {
    let json = json!({
        "type": "text",
        "text": "Response text"
    });

    let content: MessageResponseContent = common::test_roundtrip(json);
    match content {
        MessageResponseContent::Text { text } => assert_eq!(text, "Response text"),
        _ => panic!("Expected text"),
    }
}

#[test]
fn test_message_response_content_tool_use() {
    let json = json!({
        "type": "tool_use",
        "id": "tool_456",
        "name": "search",
        "input": {"query": "weather"}
    });

    let content: MessageResponseContent = common::test_roundtrip(json);
    match content {
        MessageResponseContent::ToolUse { id, name, input } => {
            assert_eq!(id, "tool_456");
            assert_eq!(name, "search");
            assert_eq!(input["query"], "weather");
        }
        _ => panic!("Expected tool_use"),
    }
}

#[test]
fn test_message_tool_choice_variants() {
    // Auto
    let auto: MessageToolChoice = serde_json::from_value(json!({"type": "auto"})).unwrap();
    assert_eq!(auto, MessageToolChoice::Auto);

    // Any
    let any: MessageToolChoice = serde_json::from_value(json!({"type": "any"})).unwrap();
    assert_eq!(any, MessageToolChoice::Any);

    // Tool
    let tool: MessageToolChoice = serde_json::from_value(json!({
        "type": "tool",
        "name": "get_weather"
    }))
    .unwrap();
    match tool {
        MessageToolChoice::Tool { name } => assert_eq!(name, "get_weather"),
        _ => panic!("Expected tool"),
    }
}

#[test]
fn test_message_tools() {
    let json = json!({
        "name": "get_weather",
        "description": "Get current weather",
        "input_schema": {
            "type": "object",
            "properties": {
                "location": {"type": "string"}
            },
            "required": ["location"]
        }
    });

    let tools: MessageTools = common::test_roundtrip(json);
    assert_eq!(tools.name, "get_weather");
    assert_eq!(tools.description, "Get current weather");
}

#[test]
fn test_message_tool_input_schema() {
    let json = json!({
        "type": "object",
        "properties": {
            "query": {"type": "string"}
        },
        "required": ["query"]
    });

    let schema: MessageToolInputSchema = common::test_roundtrip(json);
    assert_eq!(schema.schema_type, "object");
    assert_eq!(schema.required, Some(vec!["query".to_string()]));
}

#[test]
fn test_message_metadata() {
    let json = json!({
        "user_id": "user_123"
    });

    let metadata: MessageMetadata = common::test_roundtrip(json);
    assert_eq!(metadata.user_id, Some("user_123".to_string()));
}

#[test]
fn test_message_usage() {
    let json = json!({
        "input_tokens": 100,
        "output_tokens": 50,
        "cache_read_input_tokens": 20,
        "cache_creation_input_tokens": 0
    });

    let usage: MessageUsage = common::test_roundtrip(json);
    assert_eq!(usage.input_tokens, 100);
    assert_eq!(usage.output_tokens, 50);
    assert_eq!(usage.cache_read_input_tokens, 20);
}

#[test]
fn test_message_usage_default_roundtrip() {
    common::test_default_roundtrip::<MessageUsage>();
}
