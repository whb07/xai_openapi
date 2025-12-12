# xai-openapi

[![Crates.io](https://img.shields.io/crates/v/xai-openapi.svg)](https://crates.io/crates/xai-openapi)
[![Documentation](https://docs.rs/xai-openapi/badge.svg)](https://docs.rs/xai-openapi)
[![CI](https://github.com/whb07/xai_openapi/actions/workflows/ci.yml/badge.svg)](https://github.com/whb07/xai_openapi/actions/workflows/ci.yml)
[![License](https://img.shields.io/crates/l/xai-openapi.svg)](https://github.com/whb07/xai_openapi#license)

Rust types for the [xAI API](https://docs.x.ai/api), including support for Grok models.

All types are generated from the [OpenAPI specification](xai_openapi.json) included in this repository.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
xai-openapi = "0.1"
```

Or use cargo:

```bash
cargo add xai-openapi
```

## Usage

This crate provides type definitions only. You'll need to bring your own HTTP client (e.g., `reqwest`, `ureq`).

```rust
use xai_openapi::{ChatRequest, Message};

// Create a chat completion request
let request = ChatRequest {
    model: Some("grok-3".to_string()),
    messages: vec![
        Message::System {
            content: "You are a helpful assistant.".into(),
            name: None,
        },
        Message::User {
            content: "Hello!".into(),
            name: None,
        },
    ],
    ..Default::default()
};

// Serialize to JSON for your HTTP client
let json = serde_json::to_string(&request).unwrap();
```

### Tool Calling

```rust
use xai_openapi::{ChatRequest, Message, Tool, FunctionDefinition};
use serde_json::json;

let request = ChatRequest {
    model: Some("grok-3".to_string()),
    messages: vec![
        Message::User {
            content: "What's the weather in San Francisco?".into(),
            name: None,
        },
    ],
    tools: Some(vec![
        Tool::Function {
            function: FunctionDefinition {
                name: "get_weather".to_string(),
                description: Some("Get the current weather".to_string()),
                parameters: json!({
                    "type": "object",
                    "properties": {
                        "location": { "type": "string" }
                    },
                    "required": ["location"]
                }),
                strict: None,
            },
        },
    ]),
    ..Default::default()
};
```

### Embeddings

```rust
use xai_openapi::embeddings::{EmbeddingRequest, EmbeddingInput};

let request = EmbeddingRequest {
    model: Some("v1".to_string()),
    input: Some(EmbeddingInput::String("Hello, world!".to_string())),
    ..Default::default()
};
```

### Image Generation

```rust
use xai_openapi::images::GenerateImageRequest;

let request = GenerateImageRequest {
    model: Some("grok-2-image".to_string()),
    prompt: Some("A sunset over mountains".to_string()),
    n: Some(1),
    ..Default::default()
};
```

## Features

### `std` (default)

Enables standard library support. Uses `std::collections::HashMap`.

### `no_std` Support

This crate supports `no_std` environments. Disable default features and the crate will use `hashbrown::HashMap` instead:

```toml
[dependencies]
xai-openapi = { version = "0.1", default-features = false }
```

## API Coverage

| Module | Endpoint | Description |
|--------|----------|-------------|
| `chat` | `/v1/chat/completions` | Chat completions with streaming support |
| `responses` | `/v1/responses` | Responses API (OpenAI-compatible) |
| `embeddings` | `/v1/embeddings` | Text embeddings |
| `images` | `/v1/images/generations` | Image generation and editing |
| `models` | `/v1/models` | Model information and listing |
| `messages` | `/v1/messages` | Anthropic-compatible messages API |
| `search` | `/v1/documents/search` | Document search and retrieval |
| `tokenize` | `/v1/tokenize-text` | Text tokenization |
| `tools` | - | Tool/function calling types |
| `usage` | - | Token usage tracking |
| `common` | - | Shared types (Content, ImageUrl, etc.) |

## Type Conventions

All types follow these conventions:

- Derive `Clone`, `Debug`, `Default`, `PartialEq`, `Serialize`, `Deserialize`
- Optional fields use `Option<T>` with `#[serde(skip_serializing_if = "Option::is_none")]`
- Enums use appropriate serde attributes (`#[serde(tag = "type")]`, `#[serde(untagged)]`, etc.)

## Re-exports

Common types are re-exported at the crate root for convenience:

```rust
pub use chat::{ChatRequest, ChatResponse, Message};
pub use common::{Content, ImageUrl, StreamOptions};
pub use embeddings::{EmbeddingRequest, EmbeddingResponse};
pub use images::{GenerateImageRequest, GeneratedImageResponse};
pub use messages::{MessageRequest, MessageResponse};
pub use models::{LanguageModel, ListModelsResponse, Model};
pub use responses::{ModelRequest, ModelResponse};
pub use tools::{FunctionDefinition, Tool, ToolCall, ToolChoice};
pub use usage::Usage;
```

## Minimum Supported Rust Version (MSRV)

This crate requires Rust 1.75 or later.

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
