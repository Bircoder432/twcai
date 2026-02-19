//! Integration tests for TWCai

#[cfg(test)]
mod tests {
    use twcai::{CloudAIClient, types::*};

    #[test]
    fn test_client_builder() {
        let client = CloudAIClient::builder()
            .base_url("https://custom.example.com")
            .token("secret-token")
            .timeout(std::time::Duration::from_secs(30))
            .build();

        assert!(client.is_ok());
    }

    #[test]
    fn test_client_builder_missing_token() {
        let result = CloudAIClient::builder()
            .base_url("https://agent.timeweb.cloud")
            .build();

        assert!(result.is_err());
    }

    #[test]
    fn test_chat_message_creation() {
        let user_msg = ChatMessage::user("Hello, world!");
        assert!(matches!(user_msg.role, twcai::types::Role::User));

        let sys_msg = ChatMessage::system("You are a helpful assistant.");
        assert!(matches!(sys_msg.role, twcai::types::Role::System));

        let assistant_msg = ChatMessage::assistant("How can I help?");
        assert!(matches!(assistant_msg.role, twcai::types::Role::Assistant));
    }

    #[test]
    fn test_agent_call_request_default() {
        let request = AgentCallRequest::default();
        assert!(request.message.is_none());
        assert!(request.parent_message_id.is_none());
        assert!(request.file_ids.is_none());
    }

    #[test]
    fn test_chat_completion_request_default() {
        let request = ChatCompletionRequest::default();
        assert!(request.model.is_none());
        assert!(request.messages.is_empty());
        assert!(request.temperature.is_none());
    }

    #[test]
    fn test_create_conversation_request() {
        let request = CreateConversationRequest::default();
        assert!(request.items.is_none());
        assert!(request.metadata.is_none());
    }

    #[test]
    fn test_response_format_text() {
        let format = ResponseFormatText::default();
        assert_eq!(format.format_type, "text");
    }

    #[test]
    fn test_usage_calculation() {
        let usage = Usage {
            prompt_tokens: 10,
            completion_tokens: 20,
            total_tokens: 30,
        };
        assert_eq!(
            usage.total_tokens,
            usage.prompt_tokens + usage.completion_tokens
        );
    }

    #[test]
    fn test_finish_reason_serialization() {
        let reason = FinishReason::Stop;
        let json = serde_json::to_string(&reason).unwrap();
        assert_eq!(json, "\"stop\"");
    }

    #[test]
    fn test_role_serialization() {
        use twcai::types::Role;

        let user = Role::User;
        assert_eq!(serde_json::to_string(&user).unwrap(), "\"user\"");

        let assistant = Role::Assistant;
        assert_eq!(serde_json::to_string(&assistant).unwrap(), "\"assistant\"");

        let system = Role::System;
        assert_eq!(serde_json::to_string(&system).unwrap(), "\"system\"");
    }

    #[test]
    fn test_multimodal_content_creation() {
        let text_content = TextContent {
            content_type: "text".to_string(),
            text: "What's in this image?".to_string(),
        };

        let image_content = ImageUrlContent {
            content_type: "image_url".to_string(),
            image_url: ImageUrl {
                url: "https://example.com/image.jpg".to_string(),
                detail: Some("auto".to_string()),
            },
        };

        let items = vec![
            ContentItem::Text(text_content),
            ContentItem::ImageUrl(image_content),
        ];

        let msg = ChatMessage::user_multimodal(items);
        assert!(matches!(msg.role, twcai::types::Role::User));
        assert!(matches!(msg.content, ChatContent::Array(_)));
    }

    #[test]
    fn test_list_items_query_default() {
        let query = ListItemsQuery::default();
        assert!(query.after.is_none());
        assert!(query.include.is_none());
        assert!(query.limit.is_none());
        assert!(query.order.is_none());
    }

    #[test]
    fn test_get_response_query_default() {
        let query = GetResponseQuery::default();
        assert!(query.include.is_none());
        assert!(query.include_obfuscation.is_none());
        assert!(query.starting_after.is_none());
        assert!(query.stream.is_none());
    }
}
