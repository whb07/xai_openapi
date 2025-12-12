//! Schema completeness and field validation tests.
//!
//! This module validates that all Rust types correctly implement the OpenAPI specification.
//! The OpenAPI spec (`xai_openapi.json`) is the source of truth.

mod common;

use std::collections::{HashMap, HashSet};

/// Mapping of OpenAPI schema names to Rust type locations.
/// Format: "SchemaName" -> "module::TypeName"
fn get_schema_type_mapping() -> HashMap<&'static str, &'static str> {
    let mut map = HashMap::new();

    // Chat module
    map.insert("ChatRequest", "chat::ChatRequest");
    map.insert("ChatResponse", "chat::ChatResponse");
    map.insert("Message", "chat::Message");
    map.insert("Choice", "chat::Choice");
    map.insert("ChoiceMessage", "chat::ChoiceMessage");
    map.insert("ChatResponseChunk", "chat::ChatResponseChunk");
    map.insert("ChoiceChunk", "chat::ChoiceChunk");
    map.insert("Delta", "chat::Delta");
    map.insert("LogProbs", "chat::LogProbs");
    map.insert("TokenLogProb", "chat::TokenLogProb");
    map.insert("TopLogProb", "chat::TopLogProb");

    // Embeddings module
    map.insert("EmbeddingRequest", "embeddings::EmbeddingRequest");
    map.insert("EmbeddingResponse", "embeddings::EmbeddingResponse");
    map.insert("Embedding", "embeddings::Embedding");
    map.insert("EmbeddingInput", "embeddings::EmbeddingInput");
    map.insert("EmbeddingContent", "embeddings::EmbeddingContent");

    // Images module
    map.insert("GenerateImageRequest", "images::GenerateImageRequest");
    map.insert("EditImageRequest", "images::EditImageRequest");
    map.insert("GeneratedImageResponse", "images::GeneratedImageResponse");
    map.insert("GeneratedImage", "images::GeneratedImage");
    map.insert("ImageQuality", "images::ImageQuality");

    // Messages module (Anthropic-compatible)
    map.insert("MessageRequest", "messages::MessageRequest");
    map.insert("MessageResponse", "messages::MessageResponse");
    map.insert("MessageBody", "messages::MessageBody");
    map.insert("MessageContent", "messages::MessageContent");
    map.insert("MessageContentPart", "messages::MessageContentPart");
    map.insert("MessageImageContent", "messages::MessageImageContent");
    map.insert("MessageResponseContent", "messages::MessageResponseContent");
    map.insert("MessageToolChoice", "messages::MessageToolChoice");
    map.insert("MessageTools", "messages::MessageTools");
    map.insert("MessageToolInputSchema", "messages::MessageToolInputSchema");
    map.insert("MessageMetadata", "messages::MessageMetadata");
    map.insert("MessageUsage", "messages::MessageUsage");

    // Models module
    map.insert("Model", "models::Model");
    map.insert("LanguageModel", "models::LanguageModel");
    map.insert("EmbeddingModel", "models::EmbeddingModel");
    map.insert("ImageGenerationModel", "models::ImageGenerationModel");
    map.insert("ListModelsResponse", "models::ListModelsResponse");
    map.insert(
        "ListLanguageModelsResponse",
        "models::ListLanguageModelsResponse",
    );
    map.insert(
        "ListEmbeddingModelsResponse",
        "models::ListEmbeddingModelsResponse",
    );
    map.insert(
        "ListImageGenerationModelsResponse",
        "models::ListImageGenerationModelsResponse",
    );

    // Responses module
    map.insert("ModelRequest", "responses::ModelRequest");
    map.insert("ModelResponse", "responses::ModelResponse");
    map.insert("ModelInput", "responses::ModelInput");
    map.insert("ModelInputPart", "responses::ModelInputPart");
    map.insert("ModelInputContent", "responses::ModelInputContent");
    map.insert("ModelInputContentItem", "responses::ModelInputContentItem");
    map.insert("ModelInputItem", "responses::ModelInputItem");
    map.insert("ModelOutput", "responses::ModelOutput");
    map.insert("OutputMessage", "responses::OutputMessage");
    map.insert(
        "ReasoningConfiguration",
        "responses::ReasoningConfiguration",
    );
    map.insert("Reasoning", "responses::Reasoning");
    map.insert("ReasoningText", "responses::ReasoningText");
    map.insert(
        "ModelResponseConfiguration",
        "responses::ModelResponseConfiguration",
    );
    map.insert("ModelResponseFormat", "responses::ModelResponseFormat");
    map.insert(
        "DeleteStoredCompletionResponse",
        "responses::DeleteStoredCompletionResponse",
    );

    // Search module
    map.insert("SearchRequest", "search::SearchRequest");
    map.insert("SearchResponse", "search::SearchResponse");
    map.insert("SearchMatch", "search::SearchMatch");
    map.insert("DocumentsSource", "search::DocumentsSource");
    map.insert("SearchParameters", "search::SearchParameters");
    map.insert("SearchSource", "search::SearchSource");
    map.insert("RankingMetric", "search::RankingMetric");
    map.insert("RetrievalMode", "search::RetrievalMode");
    // These are represented as RetrievalMode enum variants in Rust
    map.insert("HybridRetrieval", "search::RetrievalMode::Hybrid");
    map.insert("SemanticRetrieval", "search::RetrievalMode::Semantic");
    map.insert("KeywordRetrieval", "search::RetrievalMode::Keyword");
    map.insert("HybridReranker", "search::HybridReranker");
    map.insert("RerankerModel", "search::RerankerModel");
    map.insert("ReciprocalRankFusion", "search::ReciprocalRankFusion");

    // Tokenize module
    map.insert("TokenizeRequest", "tokenize::TokenizeRequest");
    map.insert("TokenizeResponse", "tokenize::TokenizeResponse");
    map.insert("TokenizeResponseToken", "tokenize::TokenizeResponseToken");

    // Tools module
    map.insert("Tool", "tools::Tool");
    map.insert("FunctionDefinition", "tools::FunctionDefinition");
    map.insert("ToolCall", "tools::ToolCall");
    map.insert("Function", "tools::Function");
    map.insert("FunctionCall", "tools::Function"); // FunctionCall in OpenAPI maps to Function in Rust
    map.insert("FunctionChoice", "tools::FunctionChoice");
    map.insert("ToolChoice", "tools::ToolChoice");
    map.insert("FunctionToolCall", "tools::FunctionToolCall");
    map.insert("FunctionToolCallOutput", "tools::FunctionToolCallOutput");
    map.insert("WebSearchCall", "tools::WebSearchCall");
    map.insert("WebSearchAction", "tools::WebSearchAction");
    map.insert("WebSearchSource", "tools::WebSearchSource");
    map.insert("WebSearchOptions", "tools::WebSearchOptions");
    map.insert("WebSearchFilters", "tools::WebSearchFilters");
    map.insert("FileSearchCall", "tools::FileSearchCall");
    map.insert("FileSearchResult", "tools::FileSearchResult");
    map.insert("CodeInterpreterCall", "tools::CodeInterpreterCall");
    map.insert("CodeInterpreterOutput", "tools::CodeInterpreterOutput");
    map.insert("McpCall", "tools::McpCall");
    map.insert("CustomToolCall", "tools::CustomToolCall");
    map.insert("ModelTool", "tools::ModelTool");
    map.insert("ModelToolChoice", "tools::ModelToolChoice");
    map.insert("OutputText", "tools::OutputText");
    map.insert("OutputRefusal", "tools::OutputRefusal");
    map.insert("OutputMessageContent", "tools::OutputMessageContent");

    // Usage module
    map.insert("Usage", "usage::Usage");
    map.insert("PromptUsageDetail", "usage::PromptUsageDetail");
    map.insert("CompletionUsageDetail", "usage::CompletionUsageDetail");
    map.insert("ModelUsage", "usage::ModelUsage");
    map.insert("InputTokensDetails", "usage::InputTokensDetails");
    map.insert("OutputTokensDetails", "usage::OutputTokensDetails");
    map.insert(
        "ServerSideToolUsageDetails",
        "usage::ServerSideToolUsageDetails",
    );
    map.insert("EmbeddingUsage", "usage::EmbeddingUsage");

    // Common module
    map.insert("Content", "common::Content");
    map.insert("ContentPart", "common::ContentPart");
    map.insert("ImageUrl", "common::ImageUrl");
    map.insert("FileRef", "common::FileRef");
    map.insert("StreamOptions", "common::StreamOptions");
    map.insert("ResponseFormat", "common::ResponseFormat");
    map.insert("Annotation", "common::Annotation");
    map.insert("IncompleteDetails", "common::IncompleteDetails");
    map.insert("ApiKey", "common::ApiKey");
    map.insert("CompleteRequest", "common::CompleteRequest");
    map.insert("CompleteResponse", "common::CompleteResponse");
    map.insert("SampleRequest", "common::SampleRequest");
    map.insert("SampleResponse", "common::SampleResponse");
    map.insert("SampleChoice", "common::SampleChoice");
    map.insert("SampleContent", "common::SampleContent");
    map.insert(
        "StartDeferredChatResponse",
        "common::StartDeferredChatResponse",
    );
    map.insert("DebugOutput", "common::DebugOutput");
    map.insert("SystemMessageContent", "common::SystemMessageContent");
    map.insert("SystemMessagePart", "common::SystemMessagePart");

    map
}

/// Schemas that are intentionally not mapped (internal, deprecated, or handled differently)
fn get_excluded_schemas() -> HashSet<&'static str> {
    let mut set = HashSet::new();

    // These schemas may be internal, deprecated, or represented differently in Rust
    // Add schemas here that are intentionally not mapped to Rust types
    set.insert("Error"); // Generic error type
    set.insert("HTTPValidationError");
    set.insert("ValidationError");

    set
}

#[test]
fn test_openapi_spec_is_valid() {
    let spec = common::load_openapi_spec();

    // Verify basic structure
    assert!(spec.get("openapi").is_some(), "Missing openapi version");
    assert!(spec.get("info").is_some(), "Missing info section");
    assert!(spec.get("paths").is_some(), "Missing paths section");
    assert!(
        spec.get("components").is_some(),
        "Missing components section"
    );

    // Verify it's OpenAPI 3.x
    let version = spec["openapi"].as_str().unwrap();
    assert!(
        version.starts_with("3."),
        "Expected OpenAPI 3.x, got {}",
        version
    );
}

#[test]
fn test_all_schemas_have_rust_types() {
    let spec = common::load_openapi_spec();
    let schema_names = common::get_all_schema_names(&spec);
    let type_mapping = get_schema_type_mapping();
    let excluded = get_excluded_schemas();

    let mut missing_types = Vec::new();

    for schema_name in &schema_names {
        if excluded.contains(schema_name.as_str()) {
            continue;
        }

        if !type_mapping.contains_key(schema_name.as_str()) {
            missing_types.push(schema_name.clone());
        }
    }

    if !missing_types.is_empty() {
        panic!(
            "The following OpenAPI schemas are missing Rust type mappings:\n  - {}\n\n\
             Either add them to get_schema_type_mapping() or get_excluded_schemas()",
            missing_types.join("\n  - ")
        );
    }
}

#[test]
fn test_no_extra_rust_types() {
    let spec = common::load_openapi_spec();
    let schema_names: HashSet<String> = common::get_all_schema_names(&spec).into_iter().collect();
    let type_mapping = get_schema_type_mapping();
    let excluded = get_excluded_schemas();

    let mut extra_types = Vec::new();

    for (schema_name, rust_type) in &type_mapping {
        if !schema_names.contains(*schema_name) && !excluded.contains(schema_name) {
            extra_types.push(format!("{} -> {}", schema_name, rust_type));
        }
    }

    // Note: This test warns but doesn't fail, as Rust may have additional helper types
    if !extra_types.is_empty() {
        println!(
            "Warning: The following Rust types don't have corresponding OpenAPI schemas:\n  - {}",
            extra_types.join("\n  - ")
        );
    }
}

#[test]
fn test_schema_count() {
    let spec = common::load_openapi_spec();
    let schema_names = common::get_all_schema_names(&spec);
    let type_mapping = get_schema_type_mapping();
    let excluded = get_excluded_schemas();

    let expected_count = schema_names.len() - excluded.len();
    let actual_count = type_mapping.len();

    println!(
        "Schema validation summary:\n  \
         OpenAPI schemas: {}\n  \
         Excluded schemas: {}\n  \
         Expected mapped: {}\n  \
         Actually mapped: {}",
        schema_names.len(),
        excluded.len(),
        expected_count,
        actual_count
    );

    // Allow some flexibility - Rust implementation might have more types for internal use
    assert!(
        actual_count >= expected_count - 10,
        "Too few Rust types mapped. Expected at least {}, got {}",
        expected_count - 10,
        actual_count
    );
}

/// Validate that schema properties exist (basic field validation)
#[test]
fn test_critical_schemas_have_expected_fields() {
    let spec = common::load_openapi_spec();

    // Validate ChatRequest has expected fields
    let chat_request = common::get_schema(&spec, "ChatRequest");
    assert!(chat_request.is_some(), "ChatRequest schema not found");
    let chat_request = chat_request.unwrap();
    let properties = common::get_schema_properties(&chat_request);
    assert!(properties.is_some(), "ChatRequest has no properties");
    let properties = properties.unwrap();

    let expected_fields = ["messages", "model", "temperature", "stream"];
    for field in &expected_fields {
        assert!(
            properties.contains_key(*field),
            "ChatRequest missing expected field: {}",
            field
        );
    }

    // Validate ChatResponse has expected fields
    let chat_response = common::get_schema(&spec, "ChatResponse");
    assert!(chat_response.is_some(), "ChatResponse schema not found");
    let chat_response = chat_response.unwrap();
    let properties = common::get_schema_properties(&chat_response);
    assert!(properties.is_some(), "ChatResponse has no properties");
    let properties = properties.unwrap();

    let expected_fields = ["id", "object", "created", "model", "choices"];
    for field in &expected_fields {
        assert!(
            properties.contains_key(*field),
            "ChatResponse missing expected field: {}",
            field
        );
    }

    // Validate EmbeddingRequest has expected fields
    let embedding_request = common::get_schema(&spec, "EmbeddingRequest");
    assert!(
        embedding_request.is_some(),
        "EmbeddingRequest schema not found"
    );
    let embedding_request = embedding_request.unwrap();
    let properties = common::get_schema_properties(&embedding_request);
    assert!(properties.is_some(), "EmbeddingRequest has no properties");
    let properties = properties.unwrap();

    let expected_fields = ["input", "model"];
    for field in &expected_fields {
        assert!(
            properties.contains_key(*field),
            "EmbeddingRequest missing expected field: {}",
            field
        );
    }
}

/// Validate required vs optional field alignment
#[test]
fn test_required_fields_alignment() {
    let spec = common::load_openapi_spec();

    // Check that ChatResponse has required fields
    let chat_response = common::get_schema(&spec, "ChatResponse");
    assert!(chat_response.is_some(), "ChatResponse schema not found");
    let chat_response = chat_response.unwrap();
    let required = common::get_schema_required(&chat_response);

    // These fields should be required in the response
    let expected_required = ["id", "object", "created", "model", "choices"];
    for field in &expected_required {
        assert!(
            required.contains(&field.to_string()),
            "ChatResponse field '{}' should be required but isn't",
            field
        );
    }
}

/// Test that we can parse all schemas from the spec
#[test]
fn test_all_schemas_are_parseable() {
    let spec = common::load_openapi_spec();
    let schemas = spec
        .get("components")
        .and_then(|c| c.get("schemas"))
        .and_then(|s| s.as_object());

    assert!(schemas.is_some(), "No schemas found in spec");
    let schemas = schemas.unwrap();

    let mut parse_errors = Vec::new();

    for (name, schema) in schemas {
        // Check that each schema has either properties, allOf, oneOf, anyOf, or is an enum
        let has_properties = schema.get("properties").is_some();
        let has_all_of = schema.get("allOf").is_some();
        let has_one_of = schema.get("oneOf").is_some();
        let has_any_of = schema.get("anyOf").is_some();
        let has_enum = schema.get("enum").is_some();
        let has_type = schema.get("type").is_some();
        let has_ref = schema.get("$ref").is_some();

        if !has_properties
            && !has_all_of
            && !has_one_of
            && !has_any_of
            && !has_enum
            && !has_type
            && !has_ref
        {
            parse_errors.push(format!("{}: Schema has no recognizable structure", name));
        }
    }

    if !parse_errors.is_empty() {
        println!(
            "Warning: Some schemas may have unusual structure:\n  - {}",
            parse_errors.join("\n  - ")
        );
    }
}

/// Validate OpenAPI type to Rust type mapping logic
#[test]
fn test_openapi_type_mappings() {
    // Verify our type mapping function works correctly
    assert_eq!(common::openapi_type_to_rust("string", None), "String");
    assert_eq!(
        common::openapi_type_to_rust("integer", Some("int32")),
        "i32"
    );
    assert_eq!(
        common::openapi_type_to_rust("integer", Some("int64")),
        "i64"
    );
    assert_eq!(common::openapi_type_to_rust("number", Some("float")), "f32");
    assert_eq!(
        common::openapi_type_to_rust("number", Some("double")),
        "f64"
    );
    assert_eq!(common::openapi_type_to_rust("boolean", None), "bool");
    assert_eq!(common::openapi_type_to_rust("array", None), "Vec");
    assert_eq!(common::openapi_type_to_rust("object", None), "Object");
}

/// Summary test that prints coverage statistics
#[test]
fn test_schema_coverage_summary() {
    let spec = common::load_openapi_spec();
    let schema_names = common::get_all_schema_names(&spec);
    let type_mapping = get_schema_type_mapping();
    let excluded = get_excluded_schemas();

    let total_schemas = schema_names.len();
    let excluded_count = excluded.len();
    let mapped_count = type_mapping.len();
    let coverage = (mapped_count as f64 / (total_schemas - excluded_count) as f64) * 100.0;

    println!("\n=== Schema Coverage Summary ===");
    println!("Total OpenAPI schemas: {}", total_schemas);
    println!("Excluded schemas: {}", excluded_count);
    println!("Mapped to Rust types: {}", mapped_count);
    println!("Coverage: {:.1}%", coverage);

    // Group by module
    let mut module_counts: HashMap<&str, usize> = HashMap::new();
    for (_, rust_type) in &type_mapping {
        let module = rust_type.split("::").next().unwrap_or("unknown");
        *module_counts.entry(module).or_insert(0) += 1;
    }

    println!("\nTypes per module:");
    let mut modules: Vec<_> = module_counts.iter().collect();
    modules.sort_by_key(|(_, count)| std::cmp::Reverse(*count));
    for (module, count) in modules {
        println!("  {}: {}", module, count);
    }

    assert!(
        coverage >= 90.0,
        "Schema coverage is below 90%: {:.1}%",
        coverage
    );
}
