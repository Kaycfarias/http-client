# AI Agent Instructions for HTTP Client

## Architecture Overview

This is a **desktop HTTP client** built with Rust using the **Iced framework** (Elm Architecture pattern). Think Postman clone but native Rust GUI.

**Key Pattern**: Iced's TEA (The Elm Architecture)

- `App` struct = state
- `Message` enum = all possible actions
- `update()` = state transitions
- `view()` = pure rendering function

## Critical Module Boundaries

```
src/main.rs              → UI only (view functions, no business logic)
components/enums.rs      → All types/enums (single source of truth)
components/http_client.rs → HTTP logic (validates, sends, parses)
components/history.rs    → History management (max 50 items)
components/utils.rs      → Pure functions (validators, formatters)
```

**DO NOT** put HTTP logic in `main.rs` or UI code in `http_client.rs`.

## Async Runtime - CRITICAL ⚠️

**The Problem**: This panic happens when using async/reqwest without a Tokio runtime:

```
thread '<unnamed>' panicked at .../core/src/ops/function.rs:250:5:
there is no reactor running, must be called from the context of a Tokio 1.x runtime
```

**Why It Happens**: `reqwest` needs a Tokio runtime to execute async operations, but Iced doesn't automatically provide one for `Task::perform()`.

**The Solution**: Create a Tokio runtime inside `send_request()` and use `block_on()`:

```rust
use tokio::runtime::Runtime;

// ✅ CORRECT - creates runtime and blocks on async code
pub fn send_request(&self, request: HttpRequest) -> Result<HttpResponse, String> {
    let runtime = Runtime::new().unwrap();
    runtime.block_on(async {
        // async HTTP request code here
        // ...
    })
}
```

**Why This Works**:

- `send_request()` is now **synchronous** (no `async`)
- Creates a fresh Tokio runtime per request
- `block_on()` executes the async code synchronously
- Called via `Task::perform()` without `.await`:

```rust
Task::perform(
    async move { client.send_request(request) },  // No .await needed
    Message::RequestCompleted,
)
```

**Note**: Tokio is required in `Cargo.toml` with `features = ["full"]` and import `tokio::runtime::Runtime`.

## Message Flow Pattern

All state changes go through the `Message` enum:

```rust
// User types in URL input
Message::UrlChanged(url) → update() → self.url = url

// User clicks Send
Message::Submit → update() → Task::perform(async...) → Message::RequestCompleted(Result)
```

**Never** mutate `App` state directly from view functions.

## KeyValue Pattern (Headers & Query Params)

Both use `KeyValue { key, value, enabled }`. The `enabled` field allows toggling without deletion:

```rust
struct KeyValue {
    key: String,
    value: String,
    enabled: bool,  // ← checkbox controls this
}
```

When building requests, filter by `enabled`:

```rust
headers.iter().filter(|h| h.enabled && !h.key.is_empty())
```

## Validation Layers

1. **UI layer**: Disable Send button if URL empty
2. **Validation layer**: `url_validator::validate_and_normalize()` auto-adds `https://`
3. **HTTP layer**: URL parsing via `url` crate with error messages

Example:

```rust
"example.com" → "https://example.com" → valid
"not a url"   → Err("Invalid URL: ...")
```

## Building & Running

```bash
cargo run              # Dev mode
cargo build --release  # Production binary
cargo check            # Fast type checking without codegen
```

**Important**: Iced provides its own async runtime. Never add `#[tokio::main]` to `fn main()` or async methods.

## Testing Endpoints

Use these for quick testing:

```
GET  → jsonplaceholder.typicode.com/posts/1
POST → jsonplaceholder.typicode.com/posts
```

## Iced Widget Patterns

**Checkbox** in Iced 0.14+ takes only value:

```rust
checkbox(param.enabled)  // ✅ Correct
checkbox("", param.enabled)  // ❌ Old API
```

**Conditional rendering** must return same type:

```rust
if condition {
    container(text("A")).into()  // ← .into() required
} else {
    container(text("B")).into()
}
```

**Status colors** use `.color()` method directly:

```rust
text(status).color(iced::Color::from_rgb(0.0, 0.8, 0.0))
```

## History Size Limit

`RequestHistory` caps at 50 items. When adding 51st item, it truncates:

```rust
self.items.insert(0, item);  // Add to front
if self.items.len() > 50 {
    self.items.truncate(50);  // Keep newest 50
}
```

## Body Type Logic

`GET` requests auto-clear body:

```rust
if method == HTTPMethod::GET {
    self.body_type = BodyType::None;
    self.body.clear();
}
```

The UI hides body editor for GET, shows selector for POST/PUT/PATCH/DELETE.

## JSON Formatting

`utils::json_formatter::format()` uses `serde_json` for pretty print. Always check validity first:

```rust
if json_formatter::is_valid_json(&response.body) {
    let formatted = json_formatter::format(&response.body).unwrap_or(response.body.clone());
    // ...
}
```

## Error Messages

Keep them user-friendly and specific:

```rust
"URL cannot be empty"                           // ✅ Clear
"Invalid URL: relative URL without a base"      // ✅ Shows actual error
"Request failed"                                // ❌ Too vague
```

## Adding New HTTP Methods

1. Add to `HTTPMethod` enum in `enums.rs`
2. Add to `as_reqwest()` match
3. Add to `all()` Vec
4. Add to Display impl in `pick_list.rs`

## Common Pitfalls

- ❌ Using `#[tokio::main]` on methods → causes runtime panics
- ❌ Forgetting `.into()` on conditional widgets → type errors
- ❌ Direct state mutation in view functions → breaks Elm pattern
- ❌ Not filtering `enabled` field → sends disabled headers/params

## File References

- State structure: `src/main.rs:18-41` (App struct)
- All messages: `src/components/enums.rs:3-27` (Message enum)
- HTTP validation: `src/components/http_client.rs:85-98`
- History management: `src/components/history.rs:15-36`
