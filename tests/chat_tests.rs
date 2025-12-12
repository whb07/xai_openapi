//! Tests for the chat module types.

mod common;

use pretty_assertions::assert_eq;
use serde_json::json;
use xai_openapi::chat::{
    ChatRequest, ChatResponse, ChatResponseChunk, Choice, ChoiceChunk, ChoiceMessage, Delta,
    LogProbs, Message, TokenLogProb, TopLogProb,
};
use xai_openapi::common::Content;

#[test]
fn test_chat_request_minimal() {
    let json = json!({
        "messages": []
    });
    let request: ChatRequest = serde_json::from_value(json).unwrap();
    assert!(request.messages.is_empty());
    assert!(request.model.is_none());
}

#[test]
fn test_chat_request_full() {
    let json = json!({
        "model": "grok-3",
        "messages": [
            {
                "role": "system",
                "content": "You are a helpful assistant."
            },
            {
                "role": "user",
                "content": "Hello!"
            }
        ],
        "temperature": 0.7,
        "top_p": 0.9,
        "n": 1,
        "stream": false,
        "max_completion_tokens": 1000,
        "presence_penalty": 0.0,
        "frequency_penalty": 0.0,
        "seed": 42
    });

    let request: ChatRequest = serde_json::from_value(json.clone()).unwrap();
    assert_eq!(request.model, Some("grok-3".to_string()));
    assert_eq!(request.messages.len(), 2);
    assert_eq!(request.temperature, Some(0.7));
    assert_eq!(request.seed, Some(42));

    // Round-trip
    let serialized = serde_json::to_value(&request).unwrap();
    let deserialized: ChatRequest = serde_json::from_value(serialized).unwrap();
    assert_eq!(request, deserialized);
}

#[test]
fn test_chat_request_default_roundtrip() {
    common::test_default_roundtrip::<ChatRequest>();
}

#[test]
fn test_message_system() {
    let json = json!({
        "role": "system",
        "content": "You are a helpful assistant.",
        "name": "system_prompt"
    });

    let message: Message = serde_json::from_value(json).unwrap();
    match message {
        Message::System { content, name } => {
            assert_eq!(
                content,
                Content::Text("You are a helpful assistant.".to_string())
            );
            assert_eq!(name, Some("system_prompt".to_string()));
        }
        _ => panic!("Expected System message"),
    }
}

#[test]
fn test_message_user() {
    let json = json!({
        "role": "user",
        "content": "What is the weather?"
    });

    let message: Message = serde_json::from_value(json).unwrap();
    match message {
        Message::User { content, name } => {
            assert_eq!(content, Content::Text("What is the weather?".to_string()));
            assert!(name.is_none());
        }
        _ => panic!("Expected User message"),
    }
}

#[test]
fn test_message_assistant() {
    let json = json!({
        "role": "assistant",
        "content": "The weather is sunny.",
        "reasoning_content": "Based on the forecast..."
    });

    let message: Message = serde_json::from_value(json).unwrap();
    match message {
        Message::Assistant {
            content,
            reasoning_content,
            tool_calls,
            ..
        } => {
            assert_eq!(
                content,
                Some(Content::Text("The weather is sunny.".to_string()))
            );
            assert_eq!(
                reasoning_content,
                Some("Based on the forecast...".to_string())
            );
            assert!(tool_calls.is_none());
        }
        _ => panic!("Expected Assistant message"),
    }
}

#[test]
fn test_message_assistant_with_tool_calls() {
    let json = json!({
        "role": "assistant",
        "content": null,
        "tool_calls": [
            {
                "id": "call_123",
                "function": {
                    "name": "get_weather",
                    "arguments": "{\"location\": \"San Francisco\"}"
                }
            }
        ]
    });

    let message: Message = serde_json::from_value(json).unwrap();
    match message {
        Message::Assistant { tool_calls, .. } => {
            let calls = tool_calls.unwrap();
            assert_eq!(calls.len(), 1);
            assert_eq!(calls[0].id, "call_123");
            assert_eq!(calls[0].function.name, "get_weather");
        }
        _ => panic!("Expected Assistant message"),
    }
}

#[test]
fn test_message_tool() {
    let json = json!({
        "role": "tool",
        "content": "{\"temperature\": 72}",
        "tool_call_id": "call_123"
    });

    let message: Message = serde_json::from_value(json).unwrap();
    match message {
        Message::Tool {
            content,
            tool_call_id,
        } => {
            assert_eq!(content, Content::Text("{\"temperature\": 72}".to_string()));
            assert_eq!(tool_call_id, Some("call_123".to_string()));
        }
        _ => panic!("Expected Tool message"),
    }
}

#[test]
fn test_message_function_deprecated() {
    let json = json!({
        "role": "function",
        "content": "result",
        "name": "my_function"
    });

    let message: Message = serde_json::from_value(json).unwrap();
    match message {
        Message::Function { content, name } => {
            assert_eq!(content, Content::Text("result".to_string()));
            assert_eq!(name, Some("my_function".to_string()));
        }
        _ => panic!("Expected Function message"),
    }
}

#[test]
fn test_chat_response() {
    let json = json!({
        "id": "chatcmpl-123",
        "object": "chat.completion",
        "created": 1677652288,
        "model": "grok-3",
        "choices": [
            {
                "index": 0,
                "message": {
                    "role": "assistant",
                    "content": "Hello! How can I help you today?"
                },
                "finish_reason": "stop"
            }
        ],
        "usage": {
            "prompt_tokens": 10,
            "completion_tokens": 20,
            "total_tokens": 30,
            "prompt_tokens_details": {
                "text_tokens": 10,
                "audio_tokens": 0,
                "image_tokens": 0,
                "cached_tokens": 0
            },
            "completion_tokens_details": {
                "reasoning_tokens": 0,
                "audio_tokens": 0,
                "accepted_prediction_tokens": 0,
                "rejected_prediction_tokens": 0
            },
            "num_sources_used": 0
        }
    });

    let response: ChatResponse = serde_json::from_value(json).unwrap();
    assert_eq!(response.id, "chatcmpl-123");
    assert_eq!(response.object, "chat.completion");
    assert_eq!(response.model, "grok-3");
    assert_eq!(response.choices.len(), 1);
    assert_eq!(response.choices[0].finish_reason, Some("stop".to_string()));
}

#[test]
fn test_chat_response_default_roundtrip() {
    common::test_default_roundtrip::<ChatResponse>();
}

#[test]
fn test_choice() {
    let json = json!({
        "index": 0,
        "message": {
            "role": "assistant",
            "content": "Test response"
        },
        "finish_reason": "stop"
    });

    let choice: Choice = common::test_roundtrip(json);
    assert_eq!(choice.index, 0);
    assert_eq!(choice.finish_reason, Some("stop".to_string()));
}

#[test]
fn test_choice_message() {
    let json = json!({
        "role": "assistant",
        "content": "Hello!",
        "reasoning_content": "thinking...",
        "refusal": null
    });

    let message: ChoiceMessage = common::test_roundtrip(json);
    assert_eq!(message.role, "assistant");
    assert_eq!(message.content, Some("Hello!".to_string()));
    assert_eq!(message.reasoning_content, Some("thinking...".to_string()));
}

#[test]
fn test_chat_response_chunk() {
    let json = json!({
        "id": "chatcmpl-123",
        "object": "chat.completion.chunk",
        "created": 1677652288,
        "model": "grok-3",
        "choices": [
            {
                "index": 0,
                "delta": {
                    "role": "assistant",
                    "content": "Hello"
                },
                "finish_reason": null
            }
        ]
    });

    let chunk: ChatResponseChunk = common::test_roundtrip(json);
    assert_eq!(chunk.object, "chat.completion.chunk");
    assert_eq!(chunk.choices.len(), 1);
}

#[test]
fn test_choice_chunk() {
    let json = json!({
        "index": 0,
        "delta": {
            "content": " world"
        },
        "finish_reason": null
    });

    let chunk: ChoiceChunk = common::test_roundtrip(json);
    assert_eq!(chunk.delta.content, Some(" world".to_string()));
}

#[test]
fn test_delta() {
    let json = json!({
        "role": "assistant",
        "content": "Hello",
        "reasoning_content": "thinking"
    });

    let delta: Delta = common::test_roundtrip(json);
    assert_eq!(delta.role, Some("assistant".to_string()));
    assert_eq!(delta.content, Some("Hello".to_string()));
}

#[test]
fn test_logprobs() {
    let json = json!({
        "content": [
            {
                "token": "Hello",
                "logprob": -0.5,
                "top_logprobs": [
                    {"token": "Hello", "logprob": -0.5},
                    {"token": "Hi", "logprob": -1.2}
                ]
            }
        ]
    });

    let logprobs: LogProbs = common::test_roundtrip(json);
    let content = logprobs.content.unwrap();
    assert_eq!(content.len(), 1);
    assert_eq!(content[0].token, "Hello");
}

#[test]
fn test_token_logprob() {
    let json = json!({
        "token": "test",
        "logprob": -0.123,
        "top_logprobs": [],
        "bytes": [116, 101, 115, 116]
    });

    let token: TokenLogProb = common::test_roundtrip(json);
    assert_eq!(token.token, "test");
    assert_eq!(token.bytes, Some(vec![116, 101, 115, 116]));
}

#[test]
fn test_top_logprob() {
    let json = json!({
        "token": "the",
        "logprob": -0.01
    });

    let top: TopLogProb = common::test_roundtrip(json);
    assert_eq!(top.token, "the");
}

#[test]
fn test_message_default_roundtrip() {
    common::test_default_roundtrip::<Message>();
}

#[test]
fn test_all_message_variants_roundtrip() {
    // System
    let system = Message::System {
        content: "Be helpful".into(),
        name: Some("sys".to_string()),
    };
    let json = serde_json::to_value(&system).unwrap();
    let deserialized: Message = serde_json::from_value(json).unwrap();
    assert_eq!(system, deserialized);

    // User
    let user = Message::User {
        content: "Hello".into(),
        name: None,
    };
    let json = serde_json::to_value(&user).unwrap();
    let deserialized: Message = serde_json::from_value(json).unwrap();
    assert_eq!(user, deserialized);

    // Assistant
    let assistant = Message::Assistant {
        content: Some("Hi there!".into()),
        name: None,
        reasoning_content: None,
        tool_calls: None,
    };
    let json = serde_json::to_value(&assistant).unwrap();
    let deserialized: Message = serde_json::from_value(json).unwrap();
    assert_eq!(assistant, deserialized);

    // Tool
    let tool = Message::Tool {
        content: "result".into(),
        tool_call_id: Some("call_1".to_string()),
    };
    let json = serde_json::to_value(&tool).unwrap();
    let deserialized: Message = serde_json::from_value(json).unwrap();
    assert_eq!(tool, deserialized);

    // Function (deprecated)
    let function = Message::Function {
        content: "output".into(),
        name: Some("func".to_string()),
    };
    let json = serde_json::to_value(&function).unwrap();
    let deserialized: Message = serde_json::from_value(json).unwrap();
    assert_eq!(function, deserialized);
}
