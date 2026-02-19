//! Types for conversations API (OpenAI-compatible)

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Content item for conversation messages
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConversationItemContent {
    /// Content type - "input_text", "output_text", etc.
    #[serde(rename = "type")]
    pub content_type: String,
    /// Text content
    pub text: String,
}

/// Conversation item (message)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConversationItem {
    /// Item type - always "message"
    #[serde(rename = "type")]
    pub item_type: String,
    /// Unique ID of the item
    pub id: String,
    /// Status of the item
    pub status: String,
    /// Role of the message (user or assistant)
    pub role: String,
    /// Content of the item
    pub content: Vec<ConversationItemContent>,
}

/// Request to create a conversation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct CreateConversationRequest {
    /// Initial items to include in conversation context (up to 20)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<ConversationItemMessage>>,
    /// Set of 16 key-value pairs attached to the object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Value>,
}

/// Message item for creating conversation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConversationItemMessage {
    /// Item type - always "message"
    #[serde(rename = "type")]
    pub item_type: String,
    /// Role of the message (user or assistant)
    pub role: String,
    /// Content of the message
    pub content: Vec<ConversationItemContentInput>,
}

/// Input content for conversation item
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConversationItemContentInput {
    /// Content type - "input_text"
    #[serde(rename = "type")]
    pub content_type: String,
    /// Text content
    pub text: String,
}

/// Request to update a conversation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UpdateConversationRequest {
    /// Set of 16 key-value pairs attached to the object
    pub metadata: Value,
}

/// Conversation object
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Conversation {
    /// Unique ID of the conversation
    pub id: String,
    /// Object type - always "conversation"
    pub object: String,
    /// Unix timestamp of creation
    pub created_at: i64,
    /// Set of 16 key-value pairs attached to the object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Value>,
}

/// Conversation deletion confirmation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConversationDeleted {
    /// ID of the deleted conversation
    pub id: String,
    /// Object type
    pub object: String,
    /// Whether the conversation was deleted
    pub deleted: bool,
}

/// List of conversation items
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConversationItemList {
    /// Object type - always "list"
    pub object: String,
    /// List of conversation items
    pub data: Vec<ConversationItem>,
    /// ID of the first item in the list
    pub first_id: String,
    /// ID of the last item in the list
    pub last_id: String,
    /// Whether there are more items available
    pub has_more: bool,
}

/// Query parameters for listing conversation items
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ListItemsQuery {
    /// Item ID to list items after (pagination)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// Additional output data to include
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include: Option<Vec<String>>,
    /// Limit on number of objects (1-100, default 20)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// Order to return items (asc or desc)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,
}

/// Request to create items in a conversation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateItemsRequest {
    /// Items to add to the conversation (up to 20)
    pub items: Vec<CreateItemRequest>,
}

/// Single item creation request
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateItemRequest {
    /// Item type - always "message"
    #[serde(rename = "type")]
    pub item_type: String,
    /// Role of the message (user or assistant)
    pub role: String,
    /// Content of the message
    pub content: Vec<ItemContentInput>,
}

/// Content input for item creation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ItemContentInput {
    /// Content type - "input_text"
    #[serde(rename = "type")]
    pub content_type: String,
    /// Text content
    pub text: String,
}

/// Query parameters for getting a conversation item
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct GetItemQuery {
    /// Additional output data to include in model response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include: Option<Vec<String>>,
}

/// Query parameters for creating items
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct CreateItemsQuery {
    /// Additional fields to include in the response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include: Option<Vec<String>>,
}
