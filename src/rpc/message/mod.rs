use std::error::Error;

use super::interceptor::AuthInterceptor;
use super::model::Message;
use super::ycchat_auth::SignInResponse;
use super::ycchat_message::message_client::MessageClient;
use super::ycchat_message::{
    AcknowledgeMessageRequest, DeleteMessageRequest, UpdateMessageRequest,
};
use tonic::service::interceptor::InterceptedService;
use tonic::transport::Channel;
use ulid::Ulid;

mod reaction;

pub type MessageId = Ulid;

pub struct MessageService {
    client: MessageClient<InterceptedService<Channel, AuthInterceptor>>,
}

impl MessageService {
    pub async fn new(sign_in_res: SignInResponse) -> Result<Self, Box<dyn Error>> {
        let channel = Channel::from_static("http://127.0.0.1:50051")
            .connect()
            .await?;

        let client = MessageClient::with_interceptor(channel, AuthInterceptor::new(sign_in_res));

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
