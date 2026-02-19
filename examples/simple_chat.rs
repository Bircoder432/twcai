//! Simple chat completion example

use twcai::api::AgentClientExt;
use twcai::{CloudAIClient, types::*};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize client from environment variables
    // Requires TWCAI_API_TOKEN to be set
    let client = CloudAIClient::from_env()?;

    // Or build manually:
    // let client = CloudAIClient::builder()
    //     .base_url("https://agent.timeweb.cloud")
    //     .token("your-api-token")
    //     .build()?;

    let agent_id =
        std::env::var("TWCAI_AGENT_ID").expect("TWCAI_AGENT_ID environment variable not set");

    // Create chat messages
    let messages = vec![
        ChatMessage::system("You are a helpful assistant."),
        ChatMessage::user("What is the capital of France?"),
    ];

    // Build request
    let request = ChatCompletionRequest {
        model: Some("deepseek-reason".to_string()),
        messages,
        temperature: Some(0.7),
        max_completion_tokens: Some(150),
        ..Default::default()
    };

    // Send request
    let response = client.chat_completions(&agent_id, request).await?;

    // Print response
    for choice in &response.choices {
        if let ChatContent::Text(ref text) = choice.message.content {
            println!("Assistant: {}", text);
        }
    }

    println!("Usage: {} tokens", response.usage.total_tokens);

    Ok(())
}
