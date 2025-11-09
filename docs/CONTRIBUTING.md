# Contributing to BSS/OSS Rust

Thank you for your interest in contributing to the BSS/OSS Rust ecosystem! This document provides guidelines and standards for contributing.

## Code of Conduct

- Be respectful and inclusive
- Focus on constructive feedback
- Help maintain a welcoming environment

## Development Setup

1. Fork the repository
2. Clone your fork
3. Create a feature branch: `git checkout -b feature/amazing-feature`
4. Make your changes
5. Run tests: `cargo test`
6. Check formatting: `cargo fmt --check`
7. Run linter: `cargo clippy`
8. Commit your changes: `git commit -m 'Add amazing feature'`
9. Push to your branch: `git push origin feature/amazing-feature`
10. Open a Pull Request

## Coding Standards

### Rust Style

- Follow Rust standard formatting (`cargo fmt`)
- Use `cargo clippy` to catch common issues
- Write meaningful commit messages
- Add documentation comments for public APIs

### Code Organization

- Keep modules focused and cohesive
- Use workspace structure for logical separation
- Follow existing patterns in the codebase

### Testing

- Write unit tests for new functionality
- Include integration tests for API endpoints
- Aim for good test coverage

## Pull Request Process

1. Update documentation if needed
2. Ensure all tests pass
3. Update CHANGELOG.md if applicable
4. Request review from maintainers
5. Address feedback promptly

## TMF API Implementation Guidelines

When implementing new TM Forum APIs:

1. Follow the TMF Open API specification closely
2. Use the `tmf-apis-core` crate for shared types
3. Implement proper error handling
4. Add OpenAPI documentation using `utoipa`
5. Include authentication middleware
6. Write comprehensive tests

## Questions?

Feel free to open an issue for questions or clarifications.
