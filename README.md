# Wildcard Querier

A high-performance key-value store implementation with support for wildcard pattern matching and efficient analytics tracking.

## Description

Wildcard Querier is a specialized key-value store that:
- Supports fixed-length composite keys with wildcard matching
- Provides efficient increment operations for analytics tracking
- Maintains space-efficient storage using FxHashMap
- Offers flexible querying with partial wildcard patterns

## Installation

1. Add to your `Cargo.toml`:
   ```toml
   [dependencies]
   wildcard-querier = { git = "repository-url" }
   ```

2. If you're using the library with custom repeated operations, also include:
   ```toml
   [dependencies]
   repeater = { git = "repository-url" }
   ```

## Usage

Here's how to use Wildcard Querier in your Rust application:

```rust
use wildcard_querier::{graph::KeyStore, Storage, Key};

// Create a store with composite keys of length 3
let mut store = KeyStore::<3>::new();

// Increment a key
let key = ["user", "click", "button"].map(String::from);
store.increment(key)?;

// Query with wildcards
let query = [
    Key::String("user".to_string()),
    Key::Wildcard,
    Key::String("button".to_string())
];
let count = store.get(query)?;
```

### Benchmarking

Run the benchmarks to measure performance:
```bash
cargo bench
```

The benchmarks test various key sizes (5-10) for:
- Key generation
- Find-key generation
- Graph increment operations
- Graph fetch operations

## Features

- **Flexible Key Structure**: Support for fixed-length composite keys with configurable size through const generics.

- **Wildcard Pattern Matching**: Query data using wildcard patterns, allowing partial matches across key components.

- **Space-Efficient Storage**: Utilizes RustC's FxHashMap for optimal memory usage and fast lookups.

- **Performance Monitoring**: Built-in analytics trait for monitoring space usage and performance metrics.

- **Generic Implementation**: Type-safe implementation using Rust's trait system for extensibility.

## Contributing Guidelines

1. Tag issues with appropriate markers ([BUG], [FEATURE], etc.).

2. Do not start work on already assigned issues.

3. Include benchmarks for performance-critical changes.

4. Add tests for new functionality.

5. Run the full test suite before submitting PRs:
   ```bash
   cargo test && cargo bench
   ```
