# Contributing to xai-openapi

Thank you for your interest in contributing to xai-openapi! This document provides guidelines and information for contributors.

## Getting Started

1. Fork the repository
2. Clone your fork: `git clone https://github.com/YOUR_USERNAME/xai_openapi.git`
3. Create a new branch: `git checkout -b feature/your-feature-name`

## Development Setup

Ensure you have Rust 1.75 or later installed:

```bash
rustup update stable
```

Build the project:

```bash
cargo build
```

Run tests:

```bash
cargo test
```

## Code Style

This project uses standard Rust formatting and linting tools:

### Formatting

All code must be formatted with `rustfmt`:

```bash
cargo fmt
```

### Linting

All code must pass `clippy` checks with no warnings:

```bash
cargo clippy -- -D warnings
```

### Documentation

- All public items must have documentation comments
- Run `cargo doc --no-deps` to verify documentation builds correctly

## Making Changes

### Types from OpenAPI Spec

When adding or modifying types:

1. Reference the xAI OpenAPI specification
2. Use `#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]` for all structs
3. Add doc comments from the OpenAPI "description" fields
4. Use `#[serde(skip_serializing_if = "Option::is_none")]` for optional fields
5. Use `#[serde(rename = "...")]` when JSON field names differ from Rust conventions

### Commit Messages

Write clear, concise commit messages:

- Use the present tense ("Add feature" not "Added feature")
- Use the imperative mood ("Move cursor to..." not "Moves cursor to...")
- Keep the first line under 72 characters
- Reference issues and PRs in the body when relevant

Example:
```
Add ChatRequest type for chat completions

- Implements all fields from OpenAPI spec
- Includes doc comments for each field
- Adds serde attributes for proper JSON handling

Closes #123
```

## Pull Request Process

1. Ensure your code passes all checks:
   ```bash
   cargo fmt --check
   cargo clippy -- -D warnings
   cargo test
   cargo doc --no-deps
   ```

2. Update documentation if needed

3. Update CHANGELOG.md with your changes under the "Unreleased" section

4. Submit your PR with a clear description of the changes

5. Wait for CI to pass and address any review feedback

## Reporting Issues

When reporting bugs:

- Use the bug report template
- Include your Rust version (`rustc --version`)
- Provide a minimal reproduction case
- Include relevant error messages

When requesting features:

- Use the feature request template
- Explain the use case
- Consider if it aligns with the project scope (types-only, no HTTP client)

## Questions?

Feel free to open an issue for any questions about contributing.
