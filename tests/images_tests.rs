//! Tests for the images module types.

mod common;

use pretty_assertions::assert_eq;
use serde_json::json;
use xai_openapi::common::ImageUrl;
use xai_openapi::images::{
    EditImageRequest, GenerateImageRequest, GeneratedImage, GeneratedImageResponse, ImageQuality,
};

#[test]
fn test_generate_image_request_minimal() {
    let json = json!({});
    let request: GenerateImageRequest = serde_json::from_value(json).unwrap();
    assert!(request.prompt.is_none());
    assert!(request.model.is_none());
}

#[test]
fn test_generate_image_request_full() {
    let json = json!({
        "prompt": "A sunset over mountains",
        "model": "grok-2-image",
        "n": 2,
        "quality": "high",
        "response_format": "url",
        "size": "1024x1024",
        "style": "vivid",
        "user": "user123"
    });

    let request: GenerateImageRequest = common::test_roundtrip(json);
    assert_eq!(request.prompt, Some("A sunset over mountains".to_string()));
    assert_eq!(request.model, Some("grok-2-image".to_string()));
    assert_eq!(request.n, Some(2));
    assert_eq!(request.quality, Some(ImageQuality::High));
    assert_eq!(request.response_format, Some("url".to_string()));
}

#[test]
fn test_generate_image_request_default_roundtrip() {
    common::test_default_roundtrip::<GenerateImageRequest>();
}

#[test]
fn test_image_quality_variants() {
    let low: ImageQuality = serde_json::from_value(json!("low")).unwrap();
    assert_eq!(low, ImageQuality::Low);

    let medium: ImageQuality = serde_json::from_value(json!("medium")).unwrap();
    assert_eq!(medium, ImageQuality::Medium);

    let high: ImageQuality = serde_json::from_value(json!("high")).unwrap();
    assert_eq!(high, ImageQuality::High);

    // Test roundtrip
    for quality in [ImageQuality::Low, ImageQuality::Medium, ImageQuality::High] {
        let json = serde_json::to_value(&quality).unwrap();
        let deserialized: ImageQuality = serde_json::from_value(json).unwrap();
        assert_eq!(quality, deserialized);
    }
}

#[test]
fn test_image_quality_default() {
    let default = ImageQuality::default();
    assert_eq!(default, ImageQuality::Medium);
}

#[test]
fn test_edit_image_request() {
    let json = json!({
        "prompt": "Add a rainbow",
        "image": {
            "url": "https://example.com/image.png"
        },
        "model": "grok-2-image",
        "n": 1,
        "quality": "medium"
    });

    let request: EditImageRequest = common::test_roundtrip(json);
    assert_eq!(request.prompt, "Add a rainbow");
    assert_eq!(request.image.url, "https://example.com/image.png");
    assert_eq!(request.model, Some("grok-2-image".to_string()));
}

#[test]
fn test_edit_image_request_with_mask() {
    let json = json!({
        "prompt": "Replace background",
        "image": {
            "url": "https://example.com/image.png"
        },
        "mask": {
            "url": "https://example.com/mask.png",
            "detail": "high"
        }
    });

    let request: EditImageRequest = common::test_roundtrip(json);
    assert!(request.mask.is_some());
    let mask = request.mask.unwrap();
    assert_eq!(mask.url, "https://example.com/mask.png");
    assert_eq!(mask.detail, Some("high".to_string()));
}

#[test]
fn test_edit_image_request_default_roundtrip() {
    common::test_default_roundtrip::<EditImageRequest>();
}

#[test]
fn test_generated_image_response() {
    let json = json!({
        "data": [
            {
                "revised_prompt": "A beautiful sunset over majestic mountains",
                "url": "https://example.com/generated.png"
            },
            {
                "revised_prompt": "Another sunset view",
                "b64_json": "SGVsbG8gV29ybGQ="
            }
        ]
    });

    let response: GeneratedImageResponse = common::test_roundtrip(json);
    assert_eq!(response.data.len(), 2);
    assert!(response.data[0].url.is_some());
    assert!(response.data[1].b64_json.is_some());
}

#[test]
fn test_generated_image_response_default_roundtrip() {
    common::test_default_roundtrip::<GeneratedImageResponse>();
}

#[test]
fn test_generated_image() {
    let json = json!({
        "revised_prompt": "Enhanced prompt text",
        "url": "https://cdn.example.com/image.png"
    });

    let image: GeneratedImage = common::test_roundtrip(json);
    assert_eq!(image.revised_prompt, "Enhanced prompt text");
    assert_eq!(
        image.url,
        Some("https://cdn.example.com/image.png".to_string())
    );
    assert!(image.b64_json.is_none());
}

#[test]
fn test_generated_image_b64() {
    let json = json!({
        "revised_prompt": "Base64 image",
        "b64_json": "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mNk+M9QDwADhgGAWjR9awAAAABJRU5ErkJggg=="
    });

    let image: GeneratedImage = common::test_roundtrip(json);
    assert!(image.url.is_none());
    assert!(image.b64_json.is_some());
}

#[test]
fn test_image_url() {
    let json = json!({
        "url": "https://example.com/image.jpg",
        "detail": "auto"
    });

    let image_url: ImageUrl = common::test_roundtrip(json);
    assert_eq!(image_url.url, "https://example.com/image.jpg");
    assert_eq!(image_url.detail, Some("auto".to_string()));
}

#[test]
fn test_image_url_default_roundtrip() {
    common::test_default_roundtrip::<ImageUrl>();
}
