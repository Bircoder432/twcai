//! AI Agents Conversations API endpoints (OpenAI-compatible)
//!
//! Provides methods for:
//! - Creating conversations
//! - Getting conversations
//! - Updating conversations
//! - Deleting conversations
//! - Managing conversation items

use reqwest::header::AUTHORIZATION;

use crate::{
    types::*,
    CloudAIClient,
    Result,
    TwcError,
};

/// Extension trait for conversations API operations
pub trait ConversationsExt {
    /// Create a new conversation
    ///
    /// POST /api/v1/cloud-ai/agents/{agent_access_id}/v1/conversations
    fn create_conversation(
        &self,
        agent_access_id: &str,
        request: CreateConversationRequest,
    ) -> impl std::future::Future<Output = Result<Conversation>> + Send;

    /// Get an existing conversation
    ///
    /// GET /api/v1/cloud-ai/agents/{agent_access_id}/v1/conversations/{conversation_id}
    fn get_conversation(
        &self,
        agent_access_id: &str,
        conversation_id: &str,
    ) -> impl std::future::Future<Output = Result<Conversation>> + Send;

    /// Update a conversation
    ///
    /// POST /api/v1/cloud-ai/agents/{agent_access_id}/v1/conversations/{conversation_id}
    fn update_conversation(
        &self,
        agent_access_id: &str,
        conversation_id: &str,
        request: UpdateConversationRequest,
    ) -> impl std::future::Future<Output = Result<Conversation>> + Send;

    /// Delete a conversation
    ///
    /// DELETE /api/v1/cloud-ai/agents/{agent_access_id}/v1/conversations/{conversation_id}
    fn delete_conversation(
        &self,
        agent_access_id: &str,
        conversation_id: &str,
    ) -> impl std::future::Future<Output = Result<ConversationDeleted>> + Send;

    /// List items in a conversation
    ///
    /// GET /api/v1/cloud-ai/agents/{agent_access_id}/v1/conversations/{conversation_id}/items
    fn list_conversation_items(
        &self,
        agent_access_id: &str,
        conversation_id: &str,
        query: Option<ListItemsQuery>,
    ) -> impl std::future::Future<Output = Result<ConversationItemList>> + Send;

    /// Create items in a conversation
    ///
    /// POST /api/v1/cloud-ai/agents/{agent_access_id}/v1/conversations/{conversation_id}/items
    fn create_conversation_items(
        &self,
        agent_access_id: &str,
        conversation_id: &str,
        request: CreateItemsRequest,
        query: Option<CreateItemsQuery>,
    ) -> impl std::future::Future<Output = Result<ConversationItemList>> + Send;

    /// Get a specific conversation item
    ///
    /// GET /api/v1/cloud-ai/agents/{agent_access_id}/v1/conversations/{conversation_id}/items/{item_id}
    fn get_conversation_item(
        &self,
        agent_access_id: &str,
        conversation_id: &str,
        item_id: &str,
        query: Option<GetItemQuery>,
    ) -> impl std::future::Future<Output = Result<ConversationItem>> + Send;

    /// Delete a conversation item
    ///
    /// DELETE /api/v1/cloud-ai/agents/{agent_access_id}/v1/conversations/{conversation_id}/items/{item_id}
    fn delete_conversation_item(
        &self,
        agent_access_id: &str,
        conversation_id: &str,
        item_id: &str,
    ) -> impl std::future::Future<Output = Result<Conversation>> + Send;
}

impl ConversationsExt for CloudAIClient {
    async fn create_conversation(
        &self,
        agent_access_id: &str,
        request: CreateConversationRequest,
    ) -> Result<Conversation> {
        let url = format!(
            "{}/api/v1/cloud-ai/agents/{}/v1/conversations",
            self.config.base_url,
            agent_access_id
        );

        let response = self
            .config
            .http_client
            .post(&url)
            .header(AUTHORIZATION, self.config.auth_header())
            .json(&request)
            .send()
            .await
            .map_err(TwcError::Http)?;

        handle_response(response).await
    }

    async fn get_conversation(
        &self,
        agent_access_id: &str,
        conversation_id: &str,
    ) -> Result<Conversation> {
        let url = format!(
            "{}/api/v1/cloud-ai/agents/{}/v1/conversations/{}",
            self.config.base_url,
            agent_access_id,
            conversation_id
        );

        let response = self
            .config
            .http_client
            .get(&url)
            .header(AUTHORIZATION, self.config.auth_header())
            .send()
            .await
            .map_err(TwcError::Http)?;

        handle_response(response).await
    }

    async fn update_conversation(
        &self,
        agent_access_id: &str,
        conversation_id: &str,
        request: UpdateConversationRequest,
    ) -> Result<Conversation> {
        let url = format!(
            "{}/api/v1/cloud-ai/agents/{}/v1/conversations/{}",
            self.config.base_url,
            agent_access_id,
            conversation_id
        );

        let response = self
            .config
            .http_client
            .post(&url)
            .header(AUTHORIZATION, self.config.auth_header())
            .json(&request)
            .send()
            .await
            .map_err(TwcError::Http)?;

        handle_response(response).await
    }

    async fn delete_conversation(
        &self,
        agent_access_id: &str,
        conversation_id: &str,
    ) -> Result<ConversationDeleted> {
        let url = format!(
            "{}/api/v1/cloud-ai/agents/{}/v1/conversations/{}",
            self.config.base_url,
            agent_access_id,
            conversation_id
        );

        let response = self
            .config
            .http_client
            .delete(&url)
            .header(AUTHORIZATION, self.config.auth_header())
            .send()
            .await
            .map_err(TwcError::Http)?;

        handle_response(response).await
    }

    async fn list_conversation_items(
        &self,
        agent_access_id: &str,
        conversation_id: &str,
        query: Option<ListItemsQuery>,
    ) -> Result<ConversationItemList> {
        let mut url = format!(
            "{}/api/v1/cloud-ai/agents/{}/v1/conversations/{}/items",
            self.config.base_url,
            agent_access_id,
            conversation_id
        );

        if let Some(q) = query {
            let query_string = serde_urlencoded::to_string(&q)
                .map_err(|e| TwcError::InvalidRequest(e.to_string()))?;
            if !query_string.is_empty() {
                url.push('?');
                url.push_str(&query_string);
            }
        }

        let response = self
            .config
            .http_client
            .get(&url)
            .header(AUTHORIZATION, self.config.auth_header())
            .send()
            .await
            .map_err(TwcError::Http)?;

        handle_response(response).await
    }

    async fn create_conversation_items(
        &self,
        agent_access_id: &str,
        conversation_id: &str,
        request: CreateItemsRequest,
        query: Option<CreateItemsQuery>,
    ) -> Result<ConversationItemList> {
        let mut url = format!(
            "{}/api/v1/cloud-ai/agents/{}/v1/conversations/{}/items",
            self.config.base_url,
            agent_access_id,
            conversation_id
        );

        if let Some(q) = query {
            let query_string = serde_urlencoded::to_string(&q)
                .map_err(|e| TwcError::InvalidRequest(e.to_string()))?;
            if !query_string.is_empty() {
                url.push('?');
                url.push_str(&query_string);
            }
        }

        let response = self
            .config
            .http_client
            .post(&url)
            .header(AUTHORIZATION, self.config.auth_header())
            .json(&request)
            .send()
            .await
            .map_err(TwcError::Http)?;

        handle_response(response).await
    }

    async fn get_conversation_item(
        &self,
        agent_access_id: &str,
        conversation_id: &str,
        item_id: &str,
        query: Option<GetItemQuery>,
    ) -> Result<ConversationItem> {
        let mut url = format!(
            "{}/api/v1/cloud-ai/agents/{}/v1/conversations/{}/items/{}",
            self.config.base_url,
            agent_access_id,
            conversation_id,
            item_id
        );

        if let Some(q) = query {
            let query_string = serde_urlencoded::to_string(&q)
                .map_err(|e| TwcError::InvalidRequest(e.to_string()))?;
            if !query_string.is_empty() {
                url.push('?');
                url.push_str(&query_string);
            }
        }

        let response = self
            .config
            .http_client
            .get(&url)
            .header(AUTHORIZATION, self.config.auth_header())
            .send()
            .await
            .map_err(TwcError::Http)?;

        handle_response(response).await
    }

    async fn delete_conversation_item(
        &self,
        agent_access_id: &str,
        conversation_id: &str,
        item_id: &str,
    ) -> Result<Conversation> {
        let url = format!(
            "{}/api/v1/cloud-ai/agents/{}/v1/conversations/{}/items/{}",
            self.config.base_url,
            agent_access_id,
            conversation_id,
            item_id
        );

        let response = self
            .config
            .http_client
            .delete(&url)
            .header(AUTHORIZATION, self.config.auth_header())
            .send()
            .await
            .map_err(TwcError::Http)?;

        handle_response(response).await
    }
}

/// Handle HTTP response and parse JSON or return appropriate error
async fn handle_response<T: serde::de::DeserializeOwned>(
    response: reqwest::Response,
) -> Result<T> {
    let status = response.status();

    if status.is_success() {
        response.json::<T>().await.map_err(TwcError::Http)
    } else {
        let text = response.text().await.ok();
        Err(TwcError::from_status(status, text))
    }
}
