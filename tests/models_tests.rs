//! Tests for the models module types.

mod common;

use pretty_assertions::assert_eq;
use serde_json::json;
use xai_openapi::models::{
    EmbeddingModel, ImageGenerationModel, LanguageModel, ListEmbeddingModelsResponse,
    ListImageGenerationModelsResponse, ListLanguageModelsResponse, ListModelsResponse, Model,
};

#[test]
fn test_model() {
    let json = json!({
        "id": "grok-3",
        "created": 1700000000,
        "object": "model",
        "owned_by": "xai"
    });

    let model: Model = common::test_roundtrip(json);
    assert_eq!(model.id, "grok-3");
    assert_eq!(model.created, 1700000000);
    assert_eq!(model.object, "model");
    assert_eq!(model.owned_by, "xai");
}

#[test]
fn test_model_default_roundtrip() {
    common::test_default_roundtrip::<Model>();
}

#[test]
fn test_language_model() {
    let json = json!({
        "id": "grok-3",
        "fingerprint": "fp_123456",
        "created": 1700000000,
        "object": "model",
        "owned_by": "xai",
        "version": "1.0.0",
        "input_modalities": ["text", "image"],
        "output_modalities": ["text"],
        "prompt_text_token_price": 100,
        "cached_prompt_text_token_price": 50,
        "prompt_image_token_price": 200,
        "completion_text_token_price": 300,
        "search_price": 1000,
        "aliases": ["grok-latest", "grok"]
    });

    let model: LanguageModel = common::test_roundtrip(json);
    assert_eq!(model.id, "grok-3");
    assert_eq!(model.fingerprint, "fp_123456");
    assert_eq!(model.version, "1.0.0");
    assert_eq!(model.input_modalities, vec!["text", "image"]);
    assert_eq!(model.output_modalities, vec!["text"]);
    assert_eq!(model.prompt_text_token_price, 100);
    assert_eq!(model.aliases, vec!["grok-latest", "grok"]);
}

#[test]
fn test_language_model_default_roundtrip() {
    common::test_default_roundtrip::<LanguageModel>();
}

#[test]
fn test_embedding_model() {
    let json = json!({
        "id": "v1",
        "fingerprint": "fp_embed",
        "created": 1700000000,
        "object": "model",
        "owned_by": "xai",
        "version": "1.0",
        "input_modalities": ["text"],
        "output_modalities": [],
        "prompt_text_token_price": 10,
        "prompt_image_token_price": 0,
        "aliases": ["embedding-v1"]
    });

    let model: EmbeddingModel = common::test_roundtrip(json);
    assert_eq!(model.id, "v1");
    assert_eq!(model.version, "1.0");
    assert_eq!(model.input_modalities, vec!["text"]);
    assert!(model.output_modalities.is_empty());
}

#[test]
fn test_embedding_model_default_roundtrip() {
    common::test_default_roundtrip::<EmbeddingModel>();
}

#[test]
fn test_image_generation_model() {
    let json = json!({
        "id": "grok-2-image",
        "fingerprint": "fp_image",
        "max_prompt_length": 4096,
        "created": 1700000000,
        "object": "model",
        "owned_by": "xai",
        "version": "2.0",
        "input_modalities": ["text"],
        "output_modalities": ["image"],
        "image_price": 500,
        "aliases": ["grok-image"]
    });

    let model: ImageGenerationModel = common::test_roundtrip(json);
    assert_eq!(model.id, "grok-2-image");
    assert_eq!(model.max_prompt_length, 4096);
    assert_eq!(model.image_price, 500);
}

#[test]
fn test_image_generation_model_default_roundtrip() {
    common::test_default_roundtrip::<ImageGenerationModel>();
}

#[test]
fn test_list_models_response() {
    let json = json!({
        "data": [
            {
                "id": "grok-3",
                "created": 1700000000,
                "object": "model",
                "owned_by": "xai"
            },
            {
                "id": "grok-2",
                "created": 1690000000,
                "object": "model",
                "owned_by": "xai"
            }
        ],
        "object": "list"
    });

    let response: ListModelsResponse = common::test_roundtrip(json);
    assert_eq!(response.object, "list");
    assert_eq!(response.data.len(), 2);
    assert_eq!(response.data[0].id, "grok-3");
    assert_eq!(response.data[1].id, "grok-2");
}

#[test]
fn test_list_models_response_default_roundtrip() {
    common::test_default_roundtrip::<ListModelsResponse>();
}

#[test]
fn test_list_language_models_response() {
    let json = json!({
        "models": [
            {
                "id": "grok-3",
                "fingerprint": "fp_123",
                "created": 1700000000,
                "object": "model",
                "owned_by": "xai",
                "version": "3.0",
                "input_modalities": ["text"],
                "output_modalities": ["text"],
                "prompt_text_token_price": 100,
                "cached_prompt_text_token_price": 50,
                "prompt_image_token_price": 0,
                "completion_text_token_price": 200,
                "search_price": 500,
                "aliases": []
            }
        ]
    });

    let response: ListLanguageModelsResponse = common::test_roundtrip(json);
    assert_eq!(response.models.len(), 1);
    assert_eq!(response.models[0].id, "grok-3");
}

#[test]
fn test_list_language_models_response_default_roundtrip() {
    common::test_default_roundtrip::<ListLanguageModelsResponse>();
}

#[test]
fn test_list_embedding_models_response() {
    let json = json!({
        "models": [
            {
                "id": "v1",
                "fingerprint": "fp_v1",
                "created": 1700000000,
                "object": "model",
                "owned_by": "xai",
                "version": "1.0",
                "input_modalities": ["text"],
                "output_modalities": [],
                "prompt_text_token_price": 10,
                "prompt_image_token_price": 0,
                "aliases": []
            }
        ]
    });

    let response: ListEmbeddingModelsResponse = common::test_roundtrip(json);
    assert_eq!(response.models.len(), 1);
}

#[test]
fn test_list_embedding_models_response_default_roundtrip() {
    common::test_default_roundtrip::<ListEmbeddingModelsResponse>();
}

#[test]
fn test_list_image_generation_models_response() {
    let json = json!({
        "models": [
            {
                "id": "grok-2-image",
                "fingerprint": "fp_img",
                "max_prompt_length": 4096,
                "created": 1700000000,
                "object": "model",
                "owned_by": "xai",
                "version": "2.0",
                "input_modalities": [],
                "output_modalities": [],
                "image_price": 500,
                "aliases": []
            }
        ]
    });

    let response: ListImageGenerationModelsResponse = common::test_roundtrip(json);
    assert_eq!(response.models.len(), 1);
}

#[test]
fn test_list_image_generation_models_response_default_roundtrip() {
    common::test_default_roundtrip::<ListImageGenerationModelsResponse>();
}
