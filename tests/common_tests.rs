//! Tests for the common module types.

mod common;

use pretty_assertions::assert_eq;
use serde_json::json;
use xai_openapi::common::{
    Annotation, ApiKey, CompleteRequest, CompleteResponse, Content, ContentPart, DebugOutput,
    DocumentsSource, FileRef, ImageUrl, IncompleteDetails, MessageMetadata, RankingMetric,
    ResponseFormat, SampleChoice, SampleContent, SampleRequest, SampleResponse,
    StartDeferredChatResponse, StreamOptions, SystemMessageContent, SystemMessagePart,
};

#[test]
fn test_content_text() {
    let json = json!("Hello, world!");
    let content: Content = serde_json::from_value(json).unwrap();
    match content {
        Content::Text(s) => assert_eq!(s, "Hello, world!"),
        _ => panic!("Expected Text content"),
    }
}

#[test]
fn test_content_parts() {
    let json = json!([
        {
            "type": "text",
            "text": "What is in this image?"
        },
        {
            "type": "image_url",
            "image_url": {
                "url": "https://example.com/image.png"
            }
        }
    ]);

    let content: Content = serde_json::from_value(json).unwrap();
    match content {
        Content::Parts(parts) => {
            assert_eq!(parts.len(), 2);
            assert_eq!(parts[0].content_type, "text");
            assert_eq!(parts[1].content_type, "image_url");
        }
        _ => panic!("Expected Parts content"),
    }
}

#[test]
fn test_content_from_str() {
    let content: Content = "test".into();
    assert_eq!(content, Content::Text("test".to_string()));
}

#[test]
fn test_content_from_string() {
    let content: Content = String::from("test").into();
    assert_eq!(content, Content::Text("test".to_string()));
}

#[test]
fn test_content_part() {
    let json = json!({
        "type": "text",
        "text": "Hello",
        "detail": "high"
    });

    let part: ContentPart = common::test_roundtrip(json);
    assert_eq!(part.content_type, "text");
    assert_eq!(part.text, Some("Hello".to_string()));
    assert_eq!(part.detail, Some("high".to_string()));
}

#[test]
fn test_content_part_with_image() {
    let json = json!({
        "type": "image_url",
        "image_url": {
            "url": "https://example.com/img.jpg",
            "detail": "auto"
        }
    });

    let part: ContentPart = common::test_roundtrip(json);
    assert_eq!(part.content_type, "image_url");
    let img = part.image_url.unwrap();
    assert_eq!(img.url, "https://example.com/img.jpg");
    assert_eq!(img.detail, Some("auto".to_string()));
}

#[test]
fn test_content_part_with_file() {
    let json = json!({
        "type": "file",
        "file": {
            "file_id": "file_abc123"
        }
    });

    let part: ContentPart = common::test_roundtrip(json);
    assert_eq!(part.content_type, "file");
    assert_eq!(part.file.unwrap().file_id, "file_abc123");
}

#[test]
fn test_content_part_default_roundtrip() {
    common::test_default_roundtrip::<ContentPart>();
}

#[test]
fn test_image_url() {
    let json = json!({
        "url": "https://example.com/image.png",
        "detail": "high"
    });

    let img: ImageUrl = common::test_roundtrip(json);
    assert_eq!(img.url, "https://example.com/image.png");
    assert_eq!(img.detail, Some("high".to_string()));
}

#[test]
fn test_image_url_default_roundtrip() {
    common::test_default_roundtrip::<ImageUrl>();
}

#[test]
fn test_file_ref() {
    let json = json!({
        "file_id": "file_xyz789"
    });

    let file_ref: FileRef = common::test_roundtrip(json);
    assert_eq!(file_ref.file_id, "file_xyz789");
}

#[test]
fn test_stream_options() {
    let json = json!({
        "include_usage": true
    });

    let options: StreamOptions = common::test_roundtrip(json);
    assert!(options.include_usage);
}

#[test]
fn test_stream_options_default_roundtrip() {
    common::test_default_roundtrip::<StreamOptions>();
}

#[test]
fn test_response_format_text() {
    let json = json!({"type": "text"});
    let format: ResponseFormat = serde_json::from_value(json).unwrap();
    assert_eq!(format, ResponseFormat::Text);
}

#[test]
fn test_response_format_json_object() {
    let json = json!({"type": "json_object"});
    let format: ResponseFormat = serde_json::from_value(json).unwrap();
    assert_eq!(format, ResponseFormat::JsonObject);
}

#[test]
fn test_response_format_json_schema() {
    let json = json!({
        "type": "json_schema",
        "json_schema": {
            "type": "object",
            "properties": {
                "name": {"type": "string"}
            }
        }
    });

    let format: ResponseFormat = common::test_roundtrip(json);
    match format {
        ResponseFormat::JsonSchema { json_schema } => {
            assert!(json_schema.is_object());
        }
        _ => panic!("Expected JsonSchema"),
    }
}

#[test]
fn test_annotation() {
    let json = json!({
        "type": "url_citation",
        "url": "https://example.com/source",
        "title": "Source Title",
        "start_index": 10,
        "end_index": 50
    });

    let annotation: Annotation = common::test_roundtrip(json);
    assert_eq!(annotation.annotation_type, "url_citation");
    assert_eq!(annotation.url, "https://example.com/source");
    assert_eq!(annotation.title, Some("Source Title".to_string()));
    assert_eq!(annotation.start_index, Some(10));
    assert_eq!(annotation.end_index, Some(50));
}

#[test]
fn test_annotation_default_roundtrip() {
    common::test_default_roundtrip::<Annotation>();
}

#[test]
fn test_incomplete_details() {
    let json = json!({
        "reason": "max_tokens"
    });

    let details: IncompleteDetails = common::test_roundtrip(json);
    assert_eq!(details.reason, "max_tokens");
}

#[test]
fn test_api_key() {
    let json = json!({
        "redacted_api_key": "xai-***abc",
        "user_id": "user_123",
        "name": "My API Key",
        "create_time": "1700000000",
        "modify_time": "1700000001",
        "modified_by": "user_123",
        "team_id": "team_456",
        "acls": ["read", "write"],
        "api_key_id": "key_789",
        "team_blocked": false,
        "api_key_blocked": false,
        "api_key_disabled": false
    });

    let key: ApiKey = common::test_roundtrip(json);
    assert_eq!(key.redacted_api_key, "xai-***abc");
    assert_eq!(key.name, "My API Key");
    assert_eq!(key.acls, vec!["read", "write"]);
    assert!(!key.api_key_blocked);
}

#[test]
fn test_complete_request() {
    let json = json!({
        "model": "grok-3",
        "prompt": "Complete this: ",
        "max_tokens_to_sample": 100,
        "temperature": 0.5
    });

    let request: CompleteRequest = common::test_roundtrip(json);
    assert_eq!(request.model, Some("grok-3".to_string()));
    assert_eq!(request.prompt, Some("Complete this: ".to_string()));
    assert_eq!(request.max_tokens_to_sample, Some(100));
}

#[test]
fn test_complete_request_default_roundtrip() {
    common::test_default_roundtrip::<CompleteRequest>();
}

#[test]
fn test_complete_response() {
    let json = json!({
        "type": "completion",
        "id": "cmpl_123",
        "completion": "The completed text...",
        "model": "grok-3",
        "stop_reason": "stop_sequence"
    });

    let response: CompleteResponse = common::test_roundtrip(json);
    assert_eq!(response.response_type, "completion");
    assert_eq!(response.id, "cmpl_123");
    assert_eq!(response.completion, "The completed text...");
    assert_eq!(response.stop_reason, Some("stop_sequence".to_string()));
}

#[test]
fn test_complete_response_default_roundtrip() {
    common::test_default_roundtrip::<CompleteResponse>();
}

#[test]
fn test_message_metadata() {
    let json = json!({
        "user_id": "user_abc"
    });

    let metadata: MessageMetadata = common::test_roundtrip(json);
    assert_eq!(metadata.user_id, Some("user_abc".to_string()));
}

#[test]
fn test_message_metadata_default_roundtrip() {
    common::test_default_roundtrip::<MessageMetadata>();
}

#[test]
fn test_sample_request() {
    let json = json!({
        "model": "grok-3",
        "prompt": "Hello",
        "max_tokens": 50,
        "temperature": 0.7,
        "n": 2
    });

    let request: SampleRequest = common::test_roundtrip(json);
    assert_eq!(request.model, Some("grok-3".to_string()));
    assert_eq!(request.max_tokens, Some(50));
    assert_eq!(request.n, Some(2));
}

#[test]
fn test_sample_request_default_roundtrip() {
    common::test_default_roundtrip::<SampleRequest>();
}

#[test]
fn test_sample_content_text() {
    let json = json!("Simple text");
    let content: SampleContent = serde_json::from_value(json).unwrap();
    match content {
        SampleContent::Text(s) => assert_eq!(s, "Simple text"),
        _ => panic!("Expected Text"),
    }
}

#[test]
fn test_sample_content_array() {
    let json = json!(["text1", "text2"]);
    let content: SampleContent = serde_json::from_value(json).unwrap();
    match content {
        SampleContent::Array(arr) => assert_eq!(arr.len(), 2),
        _ => panic!("Expected Array"),
    }
}

#[test]
fn test_sample_response() {
    let json = json!({
        "id": "cmpl_456",
        "object": "text_completion",
        "created": 1700000000,
        "model": "grok-3",
        "choices": [
            {
                "index": 0,
                "text": "Generated text",
                "finish_reason": "stop"
            }
        ]
    });

    let response: SampleResponse = common::test_roundtrip(json);
    assert_eq!(response.id, "cmpl_456");
    assert_eq!(response.object, "text_completion");
    assert_eq!(response.choices.len(), 1);
}

#[test]
fn test_sample_response_default_roundtrip() {
    common::test_default_roundtrip::<SampleResponse>();
}

#[test]
fn test_sample_choice() {
    let json = json!({
        "index": 0,
        "text": "Output text",
        "finish_reason": "length"
    });

    let choice: SampleChoice = common::test_roundtrip(json);
    assert_eq!(choice.index, 0);
    assert_eq!(choice.text, "Output text");
    assert_eq!(choice.finish_reason, "length");
}

#[test]
fn test_start_deferred_chat_response() {
    let json = json!({
        "request_id": "req_12345"
    });

    let response: StartDeferredChatResponse = common::test_roundtrip(json);
    assert_eq!(response.request_id, "req_12345");
}

#[test]
fn test_documents_source() {
    let json = json!({
        "collection_ids": ["col_1", "col_2"]
    });

    let source: DocumentsSource = common::test_roundtrip(json);
    assert_eq!(source.collection_ids.len(), 2);
}

#[test]
fn test_ranking_metric() {
    let unknown: RankingMetric = serde_json::from_value(json!("RANKING_METRIC_UNKNOWN")).unwrap();
    assert_eq!(unknown, RankingMetric::Unknown);

    let l2: RankingMetric = serde_json::from_value(json!("RANKING_METRIC_L2_DISTANCE")).unwrap();
    assert_eq!(l2, RankingMetric::L2Distance);

    let cosine: RankingMetric =
        serde_json::from_value(json!("RANKING_METRIC_COSINE_SIMILARITY")).unwrap();
    assert_eq!(cosine, RankingMetric::CosineSimilarity);
}

#[test]
fn test_system_message_content_text() {
    let json = json!("System prompt text");
    let content: SystemMessageContent = serde_json::from_value(json).unwrap();
    match content {
        SystemMessageContent::Text(s) => assert_eq!(s, "System prompt text"),
        _ => panic!("Expected Text"),
    }
}

#[test]
fn test_system_message_content_parts() {
    let json = json!([
        {
            "type": "text",
            "text": "Part 1"
        }
    ]);

    let content: SystemMessageContent = serde_json::from_value(json).unwrap();
    match content {
        SystemMessageContent::Parts(parts) => assert_eq!(parts.len(), 1),
        _ => panic!("Expected Parts"),
    }
}

#[test]
fn test_system_message_part() {
    let json = json!({
        "type": "text",
        "text": "System text"
    });

    let part: SystemMessagePart = common::test_roundtrip(json);
    assert_eq!(part.part_type, "text");
    assert_eq!(part.text, "System text");
}

#[test]
fn test_system_message_part_default_roundtrip() {
    common::test_default_roundtrip::<SystemMessagePart>();
}

#[test]
fn test_debug_output() {
    let json = json!({
        "attempts": 1,
        "request": "{}",
        "prompt": "Test prompt",
        "engine_request": "{}",
        "responses": ["response1"],
        "chunks": [],
        "cache_read_count": 0,
        "cache_read_input_bytes": 0,
        "cache_write_count": 0,
        "cache_write_input_bytes": 0,
        "lb_address": "10.0.0.1",
        "sampler_tag": "v1",
        "sampler_checkpoint_mount": "/mnt/ckpt"
    });

    let debug: DebugOutput = common::test_roundtrip(json);
    assert_eq!(debug.attempts, 1);
    assert_eq!(debug.prompt, "Test prompt");
    assert_eq!(debug.lb_address, "10.0.0.1");
}

#[test]
fn test_debug_output_default_roundtrip() {
    common::test_default_roundtrip::<DebugOutput>();
}
