# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Repository Overview

This is a Rust library project called `docaroo-rs` that appears to be in early development. The project uses Rust 2024 edition and is set up as a library crate (not a binary).

## Key Dependencies

- **bon (3.6.4)**: Builder pattern generator for creating ergonomic APIs with compile-time checked builders
- **reqwest (0.12.20)**: HTTP client library for making web requests

These dependencies suggest this project will likely involve building an API client or documentation-related tool with HTTP capabilities.

## Common Development Commands

### Building and Checking
```bash
cargo build          # Build the library
cargo check          # Quick syntax and type checking without full compilation
cargo clean          # Remove build artifacts
```

### Testing
```bash
cargo test           # Run all tests
cargo test -- --nocapture  # Run tests with println! output visible
cargo test <test_name>     # Run a specific test
```

### Code Quality
```bash
cargo fmt            # Format code using rustfmt
cargo clippy         # Run linter for common mistakes and improvements
cargo clippy -- -W clippy::all  # Run clippy with all warnings
```

### Documentation
```bash
cargo doc            # Build documentation
cargo doc --open     # Build and open documentation in browser
```

## Project Structure

```
docaroo-rs/
├── src/
│   └── lib.rs       # Library entry point (currently contains template code)
├── Cargo.toml       # Package manifest with dependencies
└── Cargo.lock       # Dependency lock file
```

## Architecture Notes

This is a complete Rust SDK for the Docaroo Care Navigation Data API. The library provides healthcare provider pricing discovery and procedure likelihood analysis.

### Module Structure

1. **`lib.rs`**: Main entry point, re-exports public API
2. **`client.rs`**: Core `DocarooClient` with configuration and HTTP handling
3. **`models.rs`**: Request/response types with builder patterns via `bon`
4. **`error.rs`**: Comprehensive error types with retry support
5. **`pricing.rs`**: In-network pricing API operations
6. **`procedures.rs`**: Procedure likelihood API operations

### Key Design Patterns

1. **Builder Pattern**: All request types use `bon` for ergonomic construction
2. **Error Handling**: Custom error types with categorization (retryable vs non-retryable)
3. **Async/Await**: All API operations are async using Tokio
4. **Type Safety**: Strongly typed models matching the OpenAPI specification

### Testing Strategy

- Unit tests in each module using `#[cfg(test)]`
- Integration tests in `tests/` directory
- Examples in `examples/` directory demonstrating real usage

## Development Guidelines

1. When adding new functionality, consider using the `bon` crate for builder patterns when dealing with complex configurations or API requests.

2. For HTTP operations, use the `reqwest` crate which is already included as a dependency.

3. Keep tests close to the code they test by using inline test modules.

4. The project uses Rust 2024 edition, so modern Rust features and patterns are available.