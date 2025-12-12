//! Tests for the usage module types.

mod common;

use pretty_assertions::assert_eq;
use serde_json::json;
use xai_openapi::usage::{
    CompletionUsageDetail, EmbeddingUsage, InputTokensDetails, MessageUsage, ModelUsage,
    OutputTokensDetails, PromptUsageDetail, ServerSideToolUsageDetails, Usage,
};

#[test]
fn test_usage() {
    let json = json!({
        "prompt_tokens": 100,
        "completion_tokens": 50,
        "total_tokens": 150,
        "prompt_tokens_details": {
            "text_tokens": 90,
            "audio_tokens": 0,
            "image_tokens": 10,
            "cached_tokens": 20
        },
        "completion_tokens_details": {
            "reasoning_tokens": 10,
            "audio_tokens": 0,
            "accepted_prediction_tokens": 5,
            "rejected_prediction_tokens": 2
        },
        "num_sources_used": 3
    });

    let usage: Usage = common::test_roundtrip(json);
    assert_eq!(usage.prompt_tokens, 100);
    assert_eq!(usage.completion_tokens, 50);
    assert_eq!(usage.total_tokens, 150);
    assert_eq!(usage.num_sources_used, 3);
    assert_eq!(usage.prompt_tokens_details.text_tokens, 90);
    assert_eq!(usage.completion_tokens_details.reasoning_tokens, 10);
}

#[test]
fn test_usage_default_roundtrip() {
    common::test_default_roundtrip::<Usage>();
}

#[test]
fn test_prompt_usage_detail() {
    let json = json!({
        "text_tokens": 100,
        "audio_tokens": 10,
        "image_tokens": 20,
        "cached_tokens": 30
    });

    let detail: PromptUsageDetail = common::test_roundtrip(json);
    assert_eq!(detail.text_tokens, 100);
    assert_eq!(detail.audio_tokens, 10);
    assert_eq!(detail.image_tokens, 20);
    assert_eq!(detail.cached_tokens, 30);
}

#[test]
fn test_completion_usage_detail() {
    let json = json!({
        "reasoning_tokens": 50,
        "audio_tokens": 5,
        "accepted_prediction_tokens": 10,
        "rejected_prediction_tokens": 3
    });

    let detail: CompletionUsageDetail = common::test_roundtrip(json);
    assert_eq!(detail.reasoning_tokens, 50);
    assert_eq!(detail.audio_tokens, 5);
    assert_eq!(detail.accepted_prediction_tokens, 10);
    assert_eq!(detail.rejected_prediction_tokens, 3);
}

#[test]
fn test_model_usage() {
    let json = json!({
        "input_tokens": 100,
        "input_tokens_details": {
            "cached_tokens": 20
        },
        "output_tokens": 50,
        "output_tokens_details": {
            "reasoning_tokens": 10
        },
        "total_tokens": 150,
        "num_sources_used": 2,
        "num_server_side_tools_used": 1,
        "cost_in_nano_usd": 1000000
    });

    let usage: ModelUsage = common::test_roundtrip(json);
    assert_eq!(usage.input_tokens, 100);
    assert_eq!(usage.output_tokens, 50);
    assert_eq!(usage.total_tokens, 150);
    assert_eq!(usage.cost_in_nano_usd, Some(1000000));
}

#[test]
fn test_model_usage_default_roundtrip() {
    common::test_default_roundtrip::<ModelUsage>();
}

#[test]
fn test_input_tokens_details() {
    let json = json!({
        "cached_tokens": 50
    });

    let details: InputTokensDetails = common::test_roundtrip(json);
    assert_eq!(details.cached_tokens, 50);
}

#[test]
fn test_output_tokens_details() {
    let json = json!({
        "reasoning_tokens": 25
    });

    let details: OutputTokensDetails = common::test_roundtrip(json);
    assert_eq!(details.reasoning_tokens, 25);
}

#[test]
fn test_server_side_tool_usage_details() {
    let json = json!({
        "web_search_calls": 5,
        "x_search_calls": 3,
        "code_interpreter_calls": 2,
        "file_search_calls": 1,
        "mcp_calls": 0,
        "document_search_calls": 4
    });

    let details: ServerSideToolUsageDetails = common::test_roundtrip(json);
    assert_eq!(details.web_search_calls, 5);
    assert_eq!(details.x_search_calls, 3);
    assert_eq!(details.code_interpreter_calls, 2);
    assert_eq!(details.file_search_calls, 1);
    assert_eq!(details.mcp_calls, 0);
    assert_eq!(details.document_search_calls, 4);
}

#[test]
fn test_embedding_usage() {
    let json = json!({
        "prompt_tokens": 10,
        "total_tokens": 10
    });

    let usage: EmbeddingUsage = common::test_roundtrip(json);
    assert_eq!(usage.prompt_tokens, 10);
    assert_eq!(usage.total_tokens, 10);
}

#[test]
fn test_embedding_usage_default_roundtrip() {
    common::test_default_roundtrip::<EmbeddingUsage>();
}

#[test]
fn test_message_usage() {
    let json = json!({
        "input_tokens": 100,
        "cache_creation_input_tokens": 0,
        "cache_read_input_tokens": 20,
        "output_tokens": 50
    });

    let usage: MessageUsage = common::test_roundtrip(json);
    assert_eq!(usage.input_tokens, 100);
    assert_eq!(usage.cache_creation_input_tokens, 0);
    assert_eq!(usage.cache_read_input_tokens, 20);
    assert_eq!(usage.output_tokens, 50);
}

#[test]
fn test_message_usage_default_roundtrip() {
    common::test_default_roundtrip::<MessageUsage>();
}
