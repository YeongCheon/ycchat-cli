use std::error::Error;
use std::sync::Arc;

use super::interceptor::AuthMiddleware;
use super::ycchat::v1::models::Message;
use super::ycchat::v1::services::auth::SignInResponse;
use super::ycchat::v1::services::message::message_service_client::MessageServiceClient;
use super::ycchat::v1::services::message::{
    AcknowledgeMessageRequest, DeleteMessageRequest, UpdateMessageRequest,
};
use tokio::sync::Mutex;
use tonic::transport::Channel;
use tower::ServiceBuilder;
use ulid::Ulid;

mod reaction;

pub type MessageId = Ulid;

pub struct MessageService {
    client: MessageServiceClient<AuthMiddleware>,
}

impl MessageService {
    pub async fn new(auth_state: Arc<Mutex<SignInResponse>>) -> Result<Self, Box<dyn Error>> {
        let channel = Channel::from_static("http://127.0.0.1:50051")
            .connect()
            .await?;

        let auth_middleware = AuthMiddleware::new(channel.clone(), auth_state);

        let channel = ServiceBuilder::new().service(auth_middleware);

        let client = MessageServiceClient::new(channel);

        Ok(Self { client })
    }

    pub async fn acknowledge_message(
        &mut self,
        message_id: MessageId,
    ) -> Result<(), Box<dyn Error>> {
        let name = format!("message/{}", message_id);

        let request = AcknowledgeMessageRequest { name };
        self.client.acknowledge_message(request).await?;

        Ok(())
    }

    pub async fn update_message(&mut self, message: Message) -> Result<Message, Box<dyn Error>> {
        let request = UpdateMessageRequest {
            message: Some(message),
        };

        let res = self.client.update_message(request).await?;

        Ok(res.into_inner())
    }

    pub async fn delete_message(&mut self, message_id: MessageId) -> Result<(), Box<dyn Error>> {
        let name = format!("message/{}", message_id);

        let request = DeleteMessageRequest { name };

        self.client.delete_message(request).await?;

        Ok(())
    }
}
