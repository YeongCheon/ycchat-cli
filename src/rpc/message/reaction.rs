use std::error::Error;
use std::sync::Arc;

use crate::rpc::interceptor::AuthMiddleware;
use crate::rpc::model::Reaction;
use crate::rpc::ycchat_auth::SignInResponse;
use crate::rpc::ycchat_message::reaction_client::ReactionClient;
use crate::rpc::ycchat_message::{
    AddReactionRequest, DeleteReactionRequest, ListReactionsRequest, ListReactionsResponse,
};
use tokio::sync::Mutex;
use tonic::transport::Channel;
use tower::ServiceBuilder;
use ulid::Ulid;

use super::MessageId;

pub type ReactionId = Ulid;

pub struct ReactionService {
    client: ReactionClient<AuthMiddleware>,
}

impl ReactionService {
    pub async fn new(auth_state: Arc<Mutex<SignInResponse>>) -> Result<Self, Box<dyn Error>> {
        let channel = Channel::from_static("http://127.0.0.1:50051")
            .connect()
            .await?;

        let auth_middleware = AuthMiddleware::new(channel.clone(), auth_state);

        let channel = ServiceBuilder::new().service(auth_middleware);

        let client = ReactionClient::new(channel);

        Ok(Self { client })
    }

    pub async fn list_reactions(
        &mut self,
        message_id: MessageId,
    ) -> Result<ListReactionsResponse, Box<dyn Error>> {
        let parent = format!("messages/{}", message_id);

        let request = ListReactionsRequest {
            parent,
            pageable: None,
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
