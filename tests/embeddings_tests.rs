//! Tests for the embeddings module types.

mod common;

use pretty_assertions::assert_eq;
use serde_json::json;
use xai_openapi::embeddings::{
    Embedding, EmbeddingContent, EmbeddingInput, EmbeddingRequest, EmbeddingResponse,
};

#[test]
fn test_embedding_request_minimal() {
    let json = json!({});
    let request: EmbeddingRequest = serde_json::from_value(json).unwrap();
    assert!(request.input.is_none());
    assert!(request.model.is_none());
}

#[test]
fn test_embedding_request_string_input() {
    let json = json!({
        "model": "v1",
        "input": "Hello, world!",
        "encoding_format": "float"
    });

    let request: EmbeddingRequest = serde_json::from_value(json).unwrap();
    assert_eq!(request.model, Some("v1".to_string()));
    match request.input {
        Some(EmbeddingInput::String(s)) => assert_eq!(s, "Hello, world!"),
        _ => panic!("Expected String input"),
    }
}

#[test]
fn test_embedding_request_string_array_input() {
    let json = json!({
        "model": "v1",
        "input": ["Hello", "World"]
    });

    let request: EmbeddingRequest = serde_json::from_value(json).unwrap();
    match request.input {
        Some(EmbeddingInput::StringArray(arr)) => {
            assert_eq!(arr, vec!["Hello", "World"]);
        }
        _ => panic!("Expected StringArray input"),
    }
}

#[test]
fn test_embedding_request_ints_input() {
    let json = json!({
        "model": "v1",
        "input": [1, 2, 3, 4, 5]
    });

    let request: EmbeddingRequest = serde_json::from_value(json).unwrap();
    match request.input {
        Some(EmbeddingInput::Ints(arr)) => {
            assert_eq!(arr, vec![1, 2, 3, 4, 5]);
        }
        _ => panic!("Expected Ints input"),
    }
}

#[test]
fn test_embedding_request_full() {
    let json = json!({
        "model": "v1",
        "input": "test",
        "encoding_format": "base64",
        "dimensions": 1536,
        "preview": true,
        "user": "user123"
    });

    let request: EmbeddingRequest = common::test_roundtrip(json);
    assert_eq!(request.model, Some("v1".to_string()));
    assert_eq!(request.encoding_format, Some("base64".to_string()));
    assert_eq!(request.dimensions, Some(1536));
    assert_eq!(request.preview, Some(true));
    assert_eq!(request.user, Some("user123".to_string()));
}

#[test]
fn test_embedding_request_default_roundtrip() {
    common::test_default_roundtrip::<EmbeddingRequest>();
}

#[test]
fn test_embedding_response() {
    let json = json!({
        "object": "list",
        "model": "v1",
        "data": [
            {
                "index": 0,
                "embedding": [0.1, 0.2, 0.3],
                "object": "embedding"
            }
        ],
        "usage": {
            "prompt_tokens": 5,
            "total_tokens": 5
        }
    });

    let response: EmbeddingResponse = common::test_roundtrip(json);
    assert_eq!(response.object, "list");
    assert_eq!(response.model, "v1");
    assert_eq!(response.data.len(), 1);
}

#[test]
fn test_embedding_response_default_roundtrip() {
    common::test_default_roundtrip::<EmbeddingResponse>();
}

#[test]
fn test_embedding() {
    let json = json!({
        "index": 0,
        "embedding": [0.1, 0.2, 0.3, 0.4],
        "object": "embedding"
    });

    let embedding: Embedding = common::test_roundtrip(json);
    assert_eq!(embedding.index, 0);
    assert_eq!(embedding.object, "embedding");
    match embedding.embedding {
        EmbeddingContent::Float(vec) => {
            assert_eq!(vec.len(), 4);
        }
        _ => panic!("Expected Float embedding"),
    }
}

#[test]
fn test_embedding_content_float() {
    let json = json!([0.1, 0.2, 0.3]);
    let content: EmbeddingContent = serde_json::from_value(json).unwrap();
    match content {
        EmbeddingContent::Float(vec) => assert_eq!(vec.len(), 3),
        _ => panic!("Expected Float content"),
    }
}

#[test]
fn test_embedding_content_base64() {
    let json = json!("SGVsbG8gV29ybGQ=");
    let content: EmbeddingContent = serde_json::from_value(json).unwrap();
    match content {
        EmbeddingContent::Base64(s) => assert_eq!(s, "SGVsbG8gV29ybGQ="),
        _ => panic!("Expected Base64 content"),
    }
}

#[test]
fn test_embedding_input_default_roundtrip() {
    common::test_default_roundtrip::<EmbeddingInput>();
}

#[test]
fn test_embedding_content_default_roundtrip() {
    common::test_default_roundtrip::<EmbeddingContent>();
}
