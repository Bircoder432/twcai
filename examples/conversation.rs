//! Conversation management example

use twcai::{CloudAIClient, types::*};
use twcai::api::ConversationsExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = CloudAIClient::from_env()?;
    let agent_id = std::env::var("TWCAI_AGENT_ID")?;

    // Create a new conversation with initial items
    let create_request = CreateConversationRequest {
        items: Some(vec![
            ConversationItemMessage {
                item_type: "message".to_string(),
                role: "user".to_string(),
                content: vec![
                    ConversationItemContentInput {
                        content_type: "input_text".to_string(),
                        text: "Hello, let's discuss Rust programming.".to_string(),
                    }
                ],
            }
        ]),
        metadata: None,
    };

    let conversation = client.create_conversation(&agent_id, create_request).await?;
    println!("Created conversation: {}", conversation.id);

    // List items in the conversation
    let items = client.list_conversation_items(
        &agent_id,
        &conversation.id,
        Some(ListItemsQuery {
            limit: Some(10),
            ..Default::default()
        })
    ).await?;
    
    println!("Items in conversation: {}", items.data.len());

    // Add new items to the conversation
    let new_items = CreateItemsRequest {
        items: vec![
            CreateItemRequest {
                item_type: "message".to_string(),
                role: "user".to_string(),
                content: vec![
                    ItemContentInput {
                        content_type: "input_text".to_string(),
                        text: "What are the benefits of async/await?".to_string(),
                    }
                ],
            }
        ],
    };

    let updated_items = client.create_conversation_items(
        &agent_id,
        &conversation.id,
        new_items,
        None
    ).await?;
    
    println!("Added {} new items", updated_items.data.len());

    // Update conversation metadata
    let update_request = UpdateConversationRequest {
        metadata: serde_json::json!({
            "topic": "rust-async",
            "status": "active"
        }),
    };

    let updated = client.update_conversation(&agent_id, &conversation.id, update_request).await?;
    println!("Updated conversation metadata: {:?}", updated.metadata);

    // Clean up: delete the conversation
    let deleted = client.delete_conversation(&agent_id, &conversation.id).await?;
    println!("Deleted conversation: {} (success: {})", deleted.id, deleted.deleted);

    Ok(())
}
