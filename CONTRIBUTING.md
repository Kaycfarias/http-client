# Contributing to HTTP Client

Thank you for your interest in contributing to this project! 🎉

## Development Setup

### Prerequisites

- **Rust 1.75+** ([Install here](https://rustup.rs/))
- **Git**

### Clone and Build

```bash
# 1. Fork the repository on GitHub

# 2. Clone your fork
git clone https://github.com/YOUR_USERNAME/http-client.git
cd http-client

# 3. Add upstream remote
git remote add upstream https://github.com/Kaycfarias/http-client.git

# 4. Install dependencies (Linux only)
sudo apt-get install libxkbcommon-dev libwayland-dev

# 5. Build the project
cargo build

# 6. Run tests
cargo test

# 7. Run the application
cargo run
```

## Making Changes

### 1. Create a Branch

```bash
git checkout -b feat/your-feature-name
# or
git checkout -b fix/your-bug-fix
```

### 2. Make Your Changes

- Follow Rust naming conventions (`snake_case`, `PascalCase`)
- Add tests for new functionality
- Update documentation if needed
- Keep functions focused and modular

### 3. Run Quality Checks

Before committing, ensure your code passes all checks:

```bash
# Format code
cargo fmt

# Check formatting
cargo fmt -- --check

# Run linter
cargo clippy -- -D warnings

# Run tests
cargo test

# Build release
cargo build --release
```

### 4. Commit Your Changes

Use clear, descriptive commit messages:

```bash
# Good examples:
git commit -m "feat: add support for HEAD method"
git commit -m "fix: resolve timeout handling bug"
git commit -m "docs: update API documentation"
git commit -m "test: add tests for history persistence"

# Commit message format:
# <type>: <description>
#
# Types: feat, fix, docs, test, refactor, style, chore
```

### 5. Push and Create Pull Request

```bash
# Push to your fork
git push origin feat/your-feature-name

# Go to GitHub and create a Pull Request
```

## Code Style Guidelines

### Rust Conventions

- Use `snake_case` for variables, functions, modules
- Use `PascalCase` for types, structs, enums, traits
- Use `SCREAMING_SNAKE_CASE` for constants
- Add doc comments (`///`) for public APIs
- Keep line length under 100 characters

### Architecture Principles

- **Separation of Concerns**: UI in `main.rs`, logic in `components/`
- **The Elm Architecture**: Message → Update → View pattern
- **Type Safety**: Use enums instead of strings when possible
- **Error Handling**: Return `Result<T, String>` with clear messages

### Example

```rust
/// Validates and normalizes a URL, adding https:// if missing.
///
/// # Errors
/// Returns an error if the URL is empty or invalid.
pub fn validate_url(url: &str) -> Result<String, String> {
    if url.trim().is_empty() {
        return Err("URL cannot be empty".to_string());
    }

    // Implementation...
    Ok(normalized_url)
}
```

## Testing

### Running Tests

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run tests with output
cargo test -- --nocapture

# Run tests for specific module
cargo test history::tests
```

### Writing Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_something() {
        // Arrange
        let input = "test";

        // Act
        let result = function_under_test(input);

        // Assert
        assert_eq!(result, expected_value);
    }
}
```

## Project Structure

```
src/
├── main.rs                    # UI layer and application entry
├── components/
│   ├── enums.rs              # Type definitions
│   ├── http_client.rs        # HTTP logic
│   ├── history.rs            # History management
│   ├── utils.rs              # Utilities and validators
│   └── ui/                   # UI components
│       ├── header.rs
│       ├── body_editor.rs
│       └── ...
```

## Areas to Contribute

### 🐛 Bug Fixes

- Check [Issues](https://github.com/Kaycfarias/http-client/issues) for bugs
- Fix existing issues and submit PR

### ✨ New Features

Some ideas:

- Authentication support (Bearer Token, Basic Auth)
- Environment variables management
- Request collections/workspaces
- Export to cURL/Postman format
- Custom themes (Light/Dark mode)
- WebSocket support

### 📚 Documentation

- Improve README
- Add code comments
- Create usage examples
- Write tutorials

### 🧪 Testing

- Increase test coverage
- Add integration tests
- Test edge cases

## Pull Request Process

1. **Update Documentation**: If you change functionality, update relevant docs
2. **Add Tests**: New features should include tests
3. **Pass CI Checks**: Ensure all GitHub Actions checks pass
4. **One Feature per PR**: Keep PRs focused and reviewable
5. **Respond to Feedback**: Address review comments promptly

## Code of Conduct

### Our Standards

- Be respectful and inclusive
- Welcome newcomers
- Accept constructive criticism
- Focus on what's best for the project
- Show empathy towards others

## Questions?

- Open an [Issue](https://github.com/Kaycfarias/http-client/issues)
- Reach out on [LinkedIn](https://linkedin.com/in/kayc-des)

## License

By contributing, you agree that your contributions will be licensed under the MIT License.

---

**Happy Coding!** 🚀
