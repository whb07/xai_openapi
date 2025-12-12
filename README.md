# xai-openapi

[![Crates.io](https://img.shields.io/crates/v/xai-openapi.svg)](https://crates.io/crates/xai-openapi)
[![Documentation](https://docs.rs/xai-openapi/badge.svg)](https://docs.rs/xai-openapi)
[![CI](https://github.com/whb07/xai_openapi/actions/workflows/ci.yml/badge.svg)](https://github.com/whb07/xai_openapi/actions/workflows/ci.yml)
[![License](https://img.shields.io/crates/l/xai-openapi.svg)](https://github.com/whb07/xai_openapi#license)

Rust types for the [xAI API](https://docs.x.ai/api), including support for Grok models.

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
use serde_json;

// Create a chat completion request
let request = ChatRequest {
    model: Some("grok-beta".to_string()),
    messages: vec![
        // Add your messages here
    ],
    ..Default::default()
};

// Serialize to JSON for your HTTP client
let json = serde_json::to_string(&request)?;
```

## API Coverage

This crate covers the full xAI API specification:

- **Chat Completions** - `/v1/chat/completions`
- **Responses API** - `/v1/responses`
- **Embeddings** - `/v1/embeddings`
- **Image Generation** - `/v1/images/generations`
- **Models** - `/v1/models`
- **Tokenization** - `/v1/tokenize`
- **Search** - Document search and retrieval

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
