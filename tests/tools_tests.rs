//! Tests for the tools module types.

mod common;

use pretty_assertions::assert_eq;
use serde_json::json;
use xai_openapi::tools::{
    CodeInterpreterCall, CodeInterpreterOutput, CustomToolCall, FileSearchCall, FileSearchResult,
    Function, FunctionChoice, FunctionDefinition, FunctionToolCall, FunctionToolCallOutput,
    McpCall, ModelTool, ModelToolChoice, OutputMessageContent, OutputRefusal, OutputText, Tool,
    ToolCall, ToolChoice, WebSearchAction, WebSearchCall, WebSearchFilters, WebSearchOptions,
    WebSearchSource,
};

#[test]
fn test_tool_function() {
    let json = json!({
        "type": "function",
        "function": {
            "name": "get_weather",
            "parameters": {
                "type": "object",
                "properties": {
                    "location": {"type": "string"}
                }
            },
            "description": "Get current weather"
        }
    });

    let tool: Tool = common::test_roundtrip(json);
    match tool {
        Tool::Function { function } => {
            assert_eq!(function.name, "get_weather");
            assert_eq!(
                function.description,
                Some("Get current weather".to_string())
            );
        }
        _ => panic!("Expected Function tool"),
    }
}

#[test]
fn test_tool_live_search() {
    let json = json!({
        "type": "live_search",
        "sources": [
            {
                "type": "web",
                "allowed_websites": ["example.com"]
            }
        ]
    });

    let tool: Tool = common::test_roundtrip(json);
    match tool {
        Tool::LiveSearch { sources } => {
            assert_eq!(sources.len(), 1);
        }
        _ => panic!("Expected LiveSearch tool"),
    }
}

#[test]
fn test_function_definition() {
    let json = json!({
        "name": "search",
        "parameters": {"type": "object"},
        "description": "Search the web",
        "strict": true
    });

    let def: FunctionDefinition = common::test_roundtrip(json);
    assert_eq!(def.name, "search");
    assert_eq!(def.description, Some("Search the web".to_string()));
    assert_eq!(def.strict, Some(true));
}

#[test]
fn test_function_definition_default_roundtrip() {
    common::test_default_roundtrip::<FunctionDefinition>();
}

#[test]
fn test_tool_call() {
    let json = json!({
        "id": "call_123",
        "function": {
            "name": "get_weather",
            "arguments": "{\"location\": \"NYC\"}"
        },
        "index": 0,
        "type": "function"
    });

    let call: ToolCall = common::test_roundtrip(json);
    assert_eq!(call.id, "call_123");
    assert_eq!(call.function.name, "get_weather");
    assert_eq!(call.index, Some(0));
    assert_eq!(call.tool_type, Some("function".to_string()));
}

#[test]
fn test_tool_call_default_roundtrip() {
    common::test_default_roundtrip::<ToolCall>();
}

#[test]
fn test_function() {
    let json = json!({
        "name": "calculate",
        "arguments": "{\"a\": 1, \"b\": 2}"
    });

    let func: Function = common::test_roundtrip(json);
    assert_eq!(func.name, "calculate");
    assert_eq!(func.arguments, "{\"a\": 1, \"b\": 2}");
}

#[test]
fn test_function_choice() {
    let json = json!({
        "name": "specific_function"
    });

    let choice: FunctionChoice = common::test_roundtrip(json);
    assert_eq!(choice.name, "specific_function");
}

#[test]
fn test_tool_choice_mode() {
    let auto: ToolChoice = serde_json::from_value(json!("auto")).unwrap();
    match auto {
        ToolChoice::Mode(s) => assert_eq!(s, "auto"),
        _ => panic!("Expected Mode"),
    }

    let none: ToolChoice = serde_json::from_value(json!("none")).unwrap();
    match none {
        ToolChoice::Mode(s) => assert_eq!(s, "none"),
        _ => panic!("Expected Mode"),
    }
}

#[test]
fn test_tool_choice_specific() {
    let json = json!({
        "type": "function",
        "function": {"name": "my_func"}
    });

    let choice: ToolChoice = common::test_roundtrip(json);
    match choice {
        ToolChoice::Specific {
            tool_type,
            function,
        } => {
            assert_eq!(tool_type, "function");
            assert_eq!(function.unwrap().name, "my_func");
        }
        _ => panic!("Expected Specific"),
    }
}

#[test]
fn test_function_tool_call() {
    let json = json!({
        "type": "function",
        "call_id": "call_abc",
        "name": "get_data",
        "arguments": "{}",
        "id": "ftc_123",
        "status": "completed"
    });

    let call: FunctionToolCall = common::test_roundtrip(json);
    assert_eq!(call.call_type, "function");
    assert_eq!(call.call_id, "call_abc");
    assert_eq!(call.name, "get_data");
    assert_eq!(call.status, Some("completed".to_string()));
}

#[test]
fn test_function_tool_call_output() {
    let json = json!({
        "type": "function_call_output",
        "call_id": "call_xyz",
        "output": "{\"result\": true}"
    });

    let output: FunctionToolCallOutput = common::test_roundtrip(json);
    assert_eq!(output.output_type, "function_call_output");
    assert_eq!(output.call_id, "call_xyz");
    assert_eq!(output.output, "{\"result\": true}");
}

#[test]
fn test_web_search_call() {
    let json = json!({
        "type": "web_search_call",
        "action": {
            "type": "search",
            "query": "Rust programming"
        },
        "id": "ws_123",
        "status": "completed"
    });

    let call: WebSearchCall = common::test_roundtrip(json);
    assert_eq!(call.call_type, "web_search_call");
    assert_eq!(call.id, Some("ws_123".to_string()));
}

#[test]
fn test_web_search_action_search() {
    let json = json!({
        "type": "search",
        "query": "test query",
        "sources": [
            {"type": "web", "url": "https://example.com"}
        ]
    });

    let action: WebSearchAction = common::test_roundtrip(json);
    match action {
        WebSearchAction::Search { query, sources } => {
            assert_eq!(query, "test query");
            assert!(sources.is_some());
        }
        _ => panic!("Expected Search"),
    }
}

#[test]
fn test_web_search_action_open_page() {
    let json = json!({
        "type": "open_page",
        "url": "https://example.com/page"
    });

    let action: WebSearchAction = common::test_roundtrip(json);
    match action {
        WebSearchAction::OpenPage { url } => {
            assert_eq!(url, "https://example.com/page");
        }
        _ => panic!("Expected OpenPage"),
    }
}

#[test]
fn test_web_search_action_find() {
    let json = json!({
        "type": "find",
        "source": {"type": "web"},
        "pattern": "search pattern"
    });

    let action: WebSearchAction = common::test_roundtrip(json);
    match action {
        WebSearchAction::Find { pattern, .. } => {
            assert_eq!(pattern, "search pattern");
        }
        _ => panic!("Expected Find"),
    }
}

#[test]
fn test_web_search_source() {
    let json = json!({
        "type": "web",
        "url": "https://example.com"
    });

    let source: WebSearchSource = common::test_roundtrip(json);
    assert_eq!(source.source_type, "web");
    assert_eq!(source.url, Some("https://example.com".to_string()));
}

#[test]
fn test_web_search_options() {
    let json = json!({
        "search_context_size": "medium",
        "filters": {"domain": "example.com"}
    });

    let options: WebSearchOptions = common::test_roundtrip(json);
    assert_eq!(options.search_context_size, Some("medium".to_string()));
}

#[test]
fn test_web_search_filters() {
    let json = json!({
        "allowed_domains": ["example.com", "test.org"],
        "excluded_domains": ["spam.com"]
    });

    let filters: WebSearchFilters = common::test_roundtrip(json);
    assert_eq!(
        filters.allowed_domains,
        Some(vec!["example.com".to_string(), "test.org".to_string()])
    );
    assert_eq!(filters.excluded_domains, Some(vec!["spam.com".to_string()]));
}

#[test]
fn test_file_search_call() {
    let json = json!({
        "type": "file_search_call",
        "queries": ["query 1", "query 2"],
        "results": [
            {
                "file_id": "file_123",
                "filename": "doc.pdf",
                "score": 0.95,
                "text": "Relevant content..."
            }
        ],
        "id": "fs_123",
        "status": "completed"
    });

    let call: FileSearchCall = common::test_roundtrip(json);
    assert_eq!(call.call_type, "file_search_call");
    assert_eq!(call.queries.len(), 2);
    assert_eq!(call.results.len(), 1);
}

#[test]
fn test_file_search_result() {
    let json = json!({
        "file_id": "file_abc",
        "filename": "report.pdf",
        "score": 0.88,
        "text": "The content of the match"
    });

    let result: FileSearchResult = common::test_roundtrip(json);
    assert_eq!(result.file_id, "file_abc");
    assert_eq!(result.filename, "report.pdf");
    assert_eq!(result.score, 0.88);
}

#[test]
fn test_code_interpreter_call() {
    let json = json!({
        "type": "code_interpreter_call",
        "outputs": [
            {
                "type": "logs",
                "logs": "Hello World"
            }
        ],
        "code": "print('Hello World')",
        "id": "ci_123",
        "status": "completed"
    });

    let call: CodeInterpreterCall = common::test_roundtrip(json);
    assert_eq!(call.call_type, "code_interpreter_call");
    assert_eq!(call.code, Some("print('Hello World')".to_string()));
    assert_eq!(call.outputs.len(), 1);
}

#[test]
fn test_code_interpreter_output_logs() {
    let json = json!({
        "type": "logs",
        "logs": "Output text"
    });

    let output: CodeInterpreterOutput = common::test_roundtrip(json);
    match output {
        CodeInterpreterOutput::Logs { logs } => assert_eq!(logs, "Output text"),
        _ => panic!("Expected Logs"),
    }
}

#[test]
fn test_code_interpreter_output_image() {
    let json = json!({
        "type": "image",
        "url": "https://example.com/image.png"
    });

    let output: CodeInterpreterOutput = common::test_roundtrip(json);
    match output {
        CodeInterpreterOutput::Image { url } => assert_eq!(url, "https://example.com/image.png"),
        _ => panic!("Expected Image"),
    }
}

#[test]
fn test_mcp_call() {
    let json = json!({
        "type": "mcp_call",
        "name": "custom_tool",
        "server_label": "my_server",
        "arguments": "{}",
        "output": "{\"result\": 42}",
        "id": "mcp_123",
        "status": "completed"
    });

    let call: McpCall = common::test_roundtrip(json);
    assert_eq!(call.call_type, "mcp_call");
    assert_eq!(call.name, "custom_tool");
    assert_eq!(call.server_label, "my_server");
}

#[test]
fn test_custom_tool_call() {
    let json = json!({
        "type": "custom",
        "call_id": "call_abc",
        "name": "my_tool",
        "id": "ct_123",
        "input": "{\"param\": \"value\"}",
        "status": "completed"
    });

    let call: CustomToolCall = common::test_roundtrip(json);
    assert_eq!(call.call_type, "custom");
    assert_eq!(call.name, "my_tool");
}

#[test]
fn test_model_tool_function() {
    let json = json!({
        "type": "function",
        "name": "analyze",
        "parameters": {"type": "object"},
        "description": "Analyze data"
    });

    let tool: ModelTool = common::test_roundtrip(json);
    match tool {
        ModelTool::Function {
            name, description, ..
        } => {
            assert_eq!(name, "analyze");
            assert_eq!(description, Some("Analyze data".to_string()));
        }
        _ => panic!("Expected Function"),
    }
}

#[test]
fn test_model_tool_web_search() {
    let json = json!({
        "type": "web_search",
        "allowed_domains": ["example.com"],
        "search_context_size": "large"
    });

    let tool: ModelTool = common::test_roundtrip(json);
    match tool {
        ModelTool::WebSearch {
            allowed_domains,
            search_context_size,
            ..
        } => {
            assert_eq!(allowed_domains, Some(vec!["example.com".to_string()]));
            assert_eq!(search_context_size, Some("large".to_string()));
        }
        _ => panic!("Expected WebSearch"),
    }
}

#[test]
fn test_model_tool_file_search() {
    let json = json!({
        "type": "file_search",
        "vector_store_ids": ["vs_123", "vs_456"],
        "max_num_results": 10
    });

    let tool: ModelTool = common::test_roundtrip(json);
    match tool {
        ModelTool::FileSearch {
            vector_store_ids,
            max_num_results,
            ..
        } => {
            assert_eq!(vector_store_ids.len(), 2);
            assert_eq!(max_num_results, Some(10));
        }
        _ => panic!("Expected FileSearch"),
    }
}

#[test]
fn test_model_tool_choice_mode() {
    let auto: ModelToolChoice = serde_json::from_value(json!("auto")).unwrap();
    match auto {
        ModelToolChoice::Mode(s) => assert_eq!(s, "auto"),
        _ => panic!("Expected Mode"),
    }
}

#[test]
fn test_model_tool_choice_specific() {
    let json = json!({
        "type": "function",
        "name": "my_function"
    });

    let choice: ModelToolChoice = common::test_roundtrip(json);
    match choice {
        ModelToolChoice::Specific { tool_type, name } => {
            assert_eq!(tool_type, "function");
            assert_eq!(name, "my_function");
        }
        _ => panic!("Expected Specific"),
    }
}

#[test]
fn test_output_text() {
    let json = json!({
        "type": "output_text",
        "text": "Hello, world!",
        "annotations": []
    });

    let output: OutputText = common::test_roundtrip(json);
    assert_eq!(output.output_type, "output_text");
    assert_eq!(output.text, "Hello, world!");
    assert!(output.annotations.is_empty());
}

#[test]
fn test_output_refusal() {
    let json = json!({
        "type": "refusal",
        "refusal": "I cannot help with that request"
    });

    let output: OutputRefusal = common::test_roundtrip(json);
    assert_eq!(output.output_type, "refusal");
    assert_eq!(output.refusal, "I cannot help with that request");
}

#[test]
fn test_output_message_content_text() {
    let json = json!({
        "type": "output_text",
        "text": "Response text",
        "annotations": []
    });

    let content: OutputMessageContent = common::test_roundtrip(json);
    match content {
        OutputMessageContent::OutputText { text, .. } => {
            assert_eq!(text, "Response text");
        }
        _ => panic!("Expected OutputText"),
    }
}

#[test]
fn test_output_message_content_refusal() {
    let json = json!({
        "type": "refusal",
        "refusal": "Cannot comply"
    });

    let content: OutputMessageContent = common::test_roundtrip(json);
    match content {
        OutputMessageContent::Refusal { refusal } => {
            assert_eq!(refusal, "Cannot comply");
        }
        _ => panic!("Expected Refusal"),
    }
}
