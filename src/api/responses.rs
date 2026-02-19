//! AI Agents Responses API endpoints (OpenAI-compatible)
//!
//! Provides methods for:
//! - Creating responses
//! - Getting responses
//! - Deleting responses
//! - Cancelling responses

use reqwest::header::AUTHORIZATION;

use crate::{
    types::*,
    CloudAIClient,
    Result,
    TwcError,
};

/// Extension trait for responses API operations
pub trait ResponsesExt {
    /// Create a new response
    ///
    /// POST /api/v1/cloud-ai/agents/{agent_access_id}/v1/responses
    fn create_response(
        &self,
        agent_access_id: &str,
        request: CreateResponseRequest,
    ) -> impl std::future::Future<Output = Result<Response>> + Send;

    /// Get an existing response
    ///
    /// GET /api/v1/cloud-ai/agents/{agent_access_id}/v1/responses/{response_id}
    fn get_response(
        &self,
        agent_access_id: &str,
        response_id: &str,
        query: Option<GetResponseQuery>,
    ) -> impl std::future::Future<Output = Result<Response>> + Send;

    /// Delete a response
    ///
    /// DELETE /api/v1/cloud-ai/agents/{agent_access_id}/v1/responses/{response_id}
    fn delete_response(
        &self,
        agent_access_id: &str,
        response_id: &str,
    ) -> impl std::future::Future<Output = Result<()>> + Send;

    /// Cancel a response
    ///
    /// POST /api/v1/cloud-ai/agents/{agent_access_id}/v1/responses/{response_id}/cancel
    fn cancel_response(
        &self,
        agent_access_id: &str,
        response_id: &str,
    ) -> impl std::future::Future<Output = Result<Response>> + Send;
}

impl ResponsesExt for CloudAIClient {
    async fn create_response(
        &self,
        agent_access_id: &str,
        request: CreateResponseRequest,
    ) -> Result<Response> {
        let url = format!(
            "{}/api/v1/cloud-ai/agents/{}/v1/responses",
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

    async fn get_response(
        &self,
        agent_access_id: &str,
        response_id: &str,
        query: Option<GetResponseQuery>,
    ) -> Result<Response> {
        let mut url = format!(
            "{}/api/v1/cloud-ai/agents/{}/v1/responses/{}",
            self.config.base_url,
            agent_access_id,
            response_id
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

    async fn delete_response(
        &self,
        agent_access_id: &str,
        response_id: &str,
    ) -> Result<()> {
        let url = format!(
            "{}/api/v1/cloud-ai/agents/{}/v1/responses/{}",
            self.config.base_url,
            agent_access_id,
            response_id
        );

        let response = self
            .config
            .http_client
            .delete(&url)
            .header(AUTHORIZATION, self.config.auth_header())
            .send()
            .await
            .map_err(TwcError::Http)?;

        let status = response.status();
        if status.is_success() || status.as_u16() == 204 {
            Ok(())
        } else {
            Err(TwcError::from_status(status, None))
        }
    }

    async fn cancel_response(
        &self,
        agent_access_id: &str,
        response_id: &str,
    ) -> Result<Response> {
        let url = format!(
            "{}/api/v1/cloud-ai/agents/{}/v1/responses/{}/cancel",
            self.config.base_url,
            agent_access_id,
            response_id
        );

        let response = self
            .config
            .http_client
            .post(&url)
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
