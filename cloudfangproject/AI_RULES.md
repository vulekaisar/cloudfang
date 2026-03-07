# Rust AI Coding Guidelines

1.  **Safety First**: Default to Safe Rust. Only use `unsafe` when absolutely necessary and document the safety invariants clearly with `// SAFETY:` comments.
2.  **Error Handling**: Use `Result` and `Option` for error handling. Avoid `unwrap()` and `expect()` in production code unless you are 100% certain it will not panic (and document why). Use `?` operator for propagation.
3.  **Code Style**: Follow standard Rust naming conventions (snake_case for functions/variables, CamelCase for types). Use `cargo fmt` to ensure consistency.
4.  **Linting**: Pay attention to `cargo clippy` warnings and fix them. they often point to idiomatic improvements.
5.  **Testing**: Write unit tests for logic and integration tests for external interactions. Put unit tests in a `tests` module within the same file.
6.  **Documentation**: Document public APIs using doc comments (`///`).
7.  **Dependencies**: Be mindful of adding heavy dependencies. Use feature flags if only part of a crate is needed.
