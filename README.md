# TWCai

A Rust client library for the Timeweb Cloud AI API, providing OpenAI-compatible interfaces for AI agent interactions. Built with Rust 2024 Edition and native async/await support.

## Features

- OpenAI-Compatible — Drop-in replacement for OpenAI SDK with Timeweb Cloud endpoints
- Multimodal Support — Text, image, and audio inputs through unified content interfaces
- Type-Safe API — Comprehensive DTOs with serde serialization and strong typing
- Modular Design — Extension traits for clean separation of API modules
- Async/Await Throughout — Built on tokio and reqwest for high-performance async I/O

## Installation

Add this to your Cargo.toml:
```toml
[dependencies]
twcai = "0.1"
tokio = { version = "1.40", features = ["full"] }
```
## Quick Start
```rust
use twcai::{CloudAIClient, types::*};
use twcai::api::AgentClientExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize client from environment variables
    // Requires TWCAI_API_TOKEN and TWCAI_AGENT_ID to be set
    let client = CloudAIClient::from_env()?;
    let agent_id = std::env::var("TWCAI_AGENT_ID")?;

    // Create a chat completion request
    let request = ChatCompletionRequest {
        messages: vec![
            ChatMessage::system("You are a helpful assistant."),
            ChatMessage::user("What is the capital of France?"),
        ],
        temperature: Some(0.7),
        max_completion_tokens: Some(150),
        ..Default::default()
    };

    // Send request and get response
    let response = client.chat_completions(&agent_id, request).await?;
    
    for choice in &response.choices {
        if let ChatContent::Text(ref text) = choice.message.content {
            println!("Assistant: {}", text);
        }
    }

    Ok(())
}

```
## Getting Your Agent ID

1. Log in to the Timeweb Cloud Console at https://console.timeweb.cloud
2. Navigate to the AI Agents section
3. Select your agent and copy the Agent ID from the URL or agent details
4. Set the environment variable: export TWCAI_AGENT_ID="your-agent-id"

## API Modules

### Agent Client (api::AgentClientExt)

- call_agent() — Simple message-based agent interaction
- chat_completions() — OpenAI-compatible chat completions with multimodal support
- text_completions() — Legacy text completions (deprecated, use chat_completions)
- list_models() — List available models for the agent
- get_embed_code() — Get JavaScript widget embed code

### Responses (api::ResponsesExt)

- create_response() — Create a new response with advanced configuration
- get_response() — Retrieve an existing response by ID
- delete_response() — Delete a response
- cancel_response() — Cancel an in-progress response

### Conversations (api::ConversationsExt)

- create_conversation() — Create a conversation context with initial items
- get_conversation() — Retrieve conversation details
- update_conversation() — Update conversation metadata
- delete_conversation() — Delete a conversation
- list_conversation_items() — Paginated listing of conversation items
- create_conversation_items() — Add new items to a conversation
- get_conversation_item() — Retrieve a specific item
- delete_conversation_item() — Remove an item from a conversation

## Multimodal Example

Send text and image in a single message:
```rust
use twcai::{types::*, api::AgentClientExt};

let message = ChatMessage::user_multimodal(vec![
    ContentItem::Text(TextContent {
        content_type: "text".to_string(),
        text: "What's in this image?".to_string(),
    }),
    ContentItem::ImageUrl(ImageUrlContent {
        content_type: "image_url".to_string(),
        image_url: ImageUrl {
            url: "https://example.com/image.jpg".to_string(),
            detail: Some("auto".to_string()),
        },
    }),
]);

let request = ChatCompletionRequest {
    messages: vec![message],
    ..Default::default()
};
```
## Configuration

### Environment Variables

- TWCAI_API_TOKEN — Required. Your Timeweb Cloud API authentication token
- TWCAI_AGENT_ID — Required. The agent identifier from Timeweb Cloud console
- TWCAI_BASE_URL — Optional. Defaults to https://agent.timeweb.cloud

### Programmatic Configuration
```rust
use twcai::CloudAIClient;

let client = CloudAIClient::builder()
    .base_url("https://agent.timeweb.cloud")
    .token("your-api-token")
    .timeout(std::time::Duration::from_secs(120))
    .build()?;
```
## Error Handling

The library uses a comprehensive error type (TwcError) covering:

- HTTP errors (network, timeouts)
- JSON serialization/deserialization errors
- Authentication failures (401)
- Authorization failures (403)
- Resource not found (404)
- Server errors (5xx)
- Invalid request parameters

All errors implement std::error::Error and can be easily integrated with error handling frameworks.

## Examples

See the examples/ directory for complete working examples:

- simple_chat.rs — Basic chat completion
- conversation.rs — Conversation lifecycle management

### Run examples with:
```sh
export TWCAI_API_TOKEN="your-token"
export TWCAI_AGENT_ID="your-agent-id"
cargo run --example simple_chat
```
## Testing
```sh
cargo test
```
## Documentation

### Generate and open documentation:
```sh
cargo doc --open
```
## License

### Licensed under either of:

- MIT license (LICENSE-MIT or http://opensource.org/licenses/MIT)


## Acknowledgments

Built with reqwest, serde, thiserror, and tokio. Inspired by the OpenAI Rust ecosystem and designed for seamless integration with existing AI workflows.
