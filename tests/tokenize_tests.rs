//! Tests for the tokenize module types.

mod common;

use pretty_assertions::assert_eq;
use serde_json::json;
use xai_openapi::tokenize::{TokenizeRequest, TokenizeResponse, TokenizeResponseToken};

#[test]
fn test_tokenize_request_minimal() {
    let json = json!({});
    let request: TokenizeRequest = serde_json::from_value(json).unwrap();
    assert!(request.model.is_none());
    assert!(request.text.is_none());
}

#[test]
fn test_tokenize_request_full() {
    let json = json!({
        "model": "grok-3",
        "text": "Hello, world!",
        "user": "user_123"
    });

    let request: TokenizeRequest = common::test_roundtrip(json);
    assert_eq!(request.model, Some("grok-3".to_string()));
    assert_eq!(request.text, Some("Hello, world!".to_string()));
    assert_eq!(request.user, Some("user_123".to_string()));
}

#[test]
fn test_tokenize_request_default_roundtrip() {
    common::test_default_roundtrip::<TokenizeRequest>();
}

#[test]
fn test_tokenize_response() {
    let json = json!({
        "token_ids": [
            {
                "token_id": 15496,
                "string_token": "Hello",
                "token_bytes": [72, 101, 108, 108, 111]
            },
            {
                "token_id": 11,
                "string_token": ",",
                "token_bytes": [44]
            },
            {
                "token_id": 995,
                "string_token": " world",
                "token_bytes": [32, 119, 111, 114, 108, 100]
            },
            {
                "token_id": 0,
                "string_token": "!",
                "token_bytes": [33]
            }
        ]
    });

    let response: TokenizeResponse = common::test_roundtrip(json);
    assert_eq!(response.token_ids.len(), 4);
    assert_eq!(response.token_ids[0].token_id, 15496);
    assert_eq!(response.token_ids[0].string_token, "Hello");
    assert_eq!(
        response.token_ids[0].token_bytes,
        vec![72, 101, 108, 108, 111]
    );
}

#[test]
fn test_tokenize_response_default_roundtrip() {
    common::test_default_roundtrip::<TokenizeResponse>();
}

#[test]
fn test_tokenize_response_token() {
    let json = json!({
        "token_id": 1234,
        "string_token": "test",
        "token_bytes": [116, 101, 115, 116]
    });

    let token: TokenizeResponseToken = common::test_roundtrip(json);
    assert_eq!(token.token_id, 1234);
    assert_eq!(token.string_token, "test");
    assert_eq!(token.token_bytes, vec![116, 101, 115, 116]);
}

#[test]
fn test_tokenize_response_token_default_roundtrip() {
    common::test_default_roundtrip::<TokenizeResponseToken>();
}

#[test]
fn test_tokenize_empty_text() {
    let json = json!({
        "model": "grok-3",
        "text": ""
    });

    let request: TokenizeRequest = common::test_roundtrip(json);
    assert_eq!(request.text, Some("".to_string()));
}

#[test]
fn test_tokenize_response_empty() {
    let json = json!({
        "token_ids": []
    });

    let response: TokenizeResponse = common::test_roundtrip(json);
    assert!(response.token_ids.is_empty());
}

#[test]
fn test_tokenize_unicode_text() {
    let json = json!({
        "model": "grok-3",
        "text": "Hello \u{4e16}\u{754c}!"
    });

    let request: TokenizeRequest = common::test_roundtrip(json);
    assert!(request.text.unwrap().contains('\u{4e16}'));
}

#[test]
fn test_tokenize_response_token_with_special_chars() {
    let json = json!({
        "token_id": 50256,
        "string_token": "<|endoftext|>",
        "token_bytes": [60, 124, 101, 110, 100, 111, 102, 116, 101, 120, 116, 124, 62]
    });

    let token: TokenizeResponseToken = common::test_roundtrip(json);
    assert_eq!(token.string_token, "<|endoftext|>");
}
