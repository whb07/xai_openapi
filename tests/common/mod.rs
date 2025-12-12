//! Shared test utilities for xai-openapi tests.

use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value;
use std::fs;
use std::path::Path;

/// Load the OpenAPI specification from the repository root.
pub fn load_openapi_spec() -> Value {
    let spec_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("xai_openapi.json");
    let content = fs::read_to_string(&spec_path)
        .unwrap_or_else(|e| panic!("Failed to read OpenAPI spec at {:?}: {}", spec_path, e));
    serde_json::from_str(&content).unwrap_or_else(|e| panic!("Failed to parse OpenAPI spec: {}", e))
}

/// Get a schema definition from the OpenAPI spec by name.
pub fn get_schema(spec: &Value, name: &str) -> Option<Value> {
    spec.get("components")?.get("schemas")?.get(name).cloned()
}

/// Get all schema names from the OpenAPI spec.
pub fn get_all_schema_names(spec: &Value) -> Vec<String> {
    spec.get("components")
        .and_then(|c| c.get("schemas"))
        .and_then(|s| s.as_object())
        .map(|obj| obj.keys().cloned().collect())
        .unwrap_or_default()
}

/// Test round-trip serialization: value -> Rust type -> JSON -> Rust type
/// Returns the deserialized value for further assertions.
pub fn test_roundtrip<T>(json: Value) -> T
where
    T: DeserializeOwned + Serialize + PartialEq + std::fmt::Debug,
{
    // Deserialize from JSON
    let value: T = serde_json::from_value(json.clone())
        .unwrap_or_else(|e| panic!("Failed to deserialize: {}\nJSON: {}", e, json));

    // Serialize back to JSON
    let serialized =
        serde_json::to_value(&value).unwrap_or_else(|e| panic!("Failed to serialize: {}", e));

    // Deserialize again
    let roundtrip: T = serde_json::from_value(serialized.clone())
        .unwrap_or_else(|e| panic!("Failed to deserialize after roundtrip: {}", e));

    // Verify equality
    assert_eq!(
        value, roundtrip,
        "Round-trip serialization failed: values are not equal"
    );

    value
}

/// Test that a Rust type can be constructed with defaults and serialized.
pub fn test_default_roundtrip<T>()
where
    T: Default + DeserializeOwned + Serialize + PartialEq + std::fmt::Debug,
{
    let original = T::default();
    let json = serde_json::to_value(&original)
        .unwrap_or_else(|e| panic!("Failed to serialize default: {}", e));
    let deserialized: T = serde_json::from_value(json)
        .unwrap_or_else(|e| panic!("Failed to deserialize default: {}", e));
    assert_eq!(original, deserialized, "Default round-trip failed");
}

/// Extract example value from a schema if present.
pub fn get_schema_example(schema: &Value) -> Option<Value> {
    schema.get("example").cloned()
}

/// Get the properties of a schema.
pub fn get_schema_properties(schema: &Value) -> Option<&serde_json::Map<String, Value>> {
    schema.get("properties").and_then(|p| p.as_object())
}

/// Get the required fields of a schema.
pub fn get_schema_required(schema: &Value) -> Vec<String> {
    schema
        .get("required")
        .and_then(|r| r.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(String::from))
                .collect()
        })
        .unwrap_or_default()
}

/// Map OpenAPI type to expected Rust type name.
pub fn openapi_type_to_rust(openapi_type: &str, format: Option<&str>) -> &'static str {
    match (openapi_type, format) {
        ("string", _) => "String",
        ("integer", Some("int32")) => "i32",
        ("integer", Some("int64")) => "i64",
        ("integer", _) => "i32",
        ("number", Some("float")) => "f32",
        ("number", Some("double")) => "f64",
        ("number", _) => "f64",
        ("boolean", _) => "bool",
        ("array", _) => "Vec",
        ("object", _) => "Object",
        _ => "Unknown",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_openapi_spec() {
        let spec = load_openapi_spec();
        assert!(spec.get("openapi").is_some(), "Missing openapi version");
        assert!(spec.get("info").is_some(), "Missing info section");
        assert!(
            spec.get("components").is_some(),
            "Missing components section"
        );
    }

    #[test]
    fn test_get_all_schema_names() {
        let spec = load_openapi_spec();
        let names = get_all_schema_names(&spec);
        assert!(!names.is_empty(), "No schemas found in spec");
    }
}
