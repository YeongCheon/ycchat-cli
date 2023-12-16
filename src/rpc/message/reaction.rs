use std::error::Error;
use std::sync::Arc;

use crate::rpc::interceptor::AuthMiddleware;
use crate::rpc::ycchat::v1::models::Reaction;
use crate::rpc::ycchat::v1::services::auth::SignInResponse;
use crate::rpc::ycchat::v1::services::message::reaction_service_client::ReactionServiceClient;
use crate::rpc::ycchat::v1::services::message::{
    AddReactionRequest, DeleteReactionRequest, ListReactionsRequest, ListReactionsResponse,
};
use tokio::sync::Mutex;
use tonic::transport::Channel;
use tower::ServiceBuilder;
use ulid::Ulid;

use super::MessageId;

pub type ReactionId = Ulid;

pub struct ReactionService {
    client: ReactionServiceClient<AuthMiddleware>,
}

impl ReactionService {
    pub async fn new(auth_state: Arc<Mutex<SignInResponse>>) -> Result<Self, Box<dyn Error>> {
        let channel = Channel::from_static("http://127.0.0.1:50051")
            .connect()
            .await?;

        let auth_middleware = AuthMiddleware::new(channel.clone(), auth_state);

        let channel = ServiceBuilder::new().service(auth_middleware);

        let client = ReactionServiceClient::new(channel);

        Ok(Self { client })
    }

    pub async fn list_reactions(
        &mut self,
        message_id: MessageId,
        page_size: i32,
        page_token: Option<String>,
    ) -> Result<ListReactionsResponse, Box<dyn Error>> {
        let parent = format!("messages/{}", message_id);

        let request = ListReactionsRequest {
            parent,
            page_size,
            page_token,
        };

        let res = self.client.list_reactions(request).await?;

        Ok(res.into_inner())
    }

    pub async fn add_reaction(
        &mut self,
        message_id: MessageId,
    ) -> Result<Reaction, Box<dyn Error>> {
        let parent = format!("messages/{}", message_id);

        let request = AddReactionRequest { parent };

        let res = self.client.add_reaction(request).await?;

        Ok(res.into_inner())
    }

    pub async fn delete_reaction(&mut self, reaction_id: ReactionId) -> Result<(), Box<dyn Error>> {
        let name = format!("reactions/{}", reaction_id);
        let request = DeleteReactionRequest { name };

        self.client.delete_reaction(request).await?;

        Ok(())
    }
}
