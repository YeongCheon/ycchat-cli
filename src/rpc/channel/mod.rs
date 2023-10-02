use std::error::Error;

use super::interceptor::AuthInterceptor;
use super::message::MessageId;
use super::model::Channel;
use super::server::ServerId;
use super::ycchat_auth::SignInResponse;
use super::ycchat_channel::channel_client::ChannelClient;
use super::ycchat_channel::{
    CreateChannelRequest, DeleteChannelRequest, ListChannelMembersRequest,
    ListChannelMembersResponse, ListChannelMessagesRequest, ListServerChannelsRequest,
    ListServerChannelsResponse, SpeechRequest, SpeechResponse, UpdateChannelRequest,
};
use tonic::service::interceptor::InterceptedService;
use tonic::transport::Channel as TonicChannel;
use ulid::Ulid;

pub type ChannelId = Ulid;

struct ChannelService {
    client: ChannelClient<InterceptedService<TonicChannel, AuthInterceptor>>,
}

impl ChannelService {
    pub async fn new(sign_in_res: SignInResponse) -> Result<Self, Box<dyn Error>> {
        let channel = TonicChannel::from_static("http://127.0.0.1:50051")
            .connect()
            .await?;

        let client = ChannelClient::with_interceptor(channel, AuthInterceptor::new(sign_in_res));

        Ok(Self { client })
    }

    pub async fn list_server_channels(
        &mut self,
        server_id: ServerId,
    ) -> Result<ListServerChannelsResponse, Box<dyn Error>> {
        let parent = format!("servers/{}/channels", server_id);

        let request = ListServerChannelsRequest {
            parent,
            pageable: None,
        };
        let res = self.client.list_server_channels(request).await?;

        Ok(res.into_inner())
    }

    pub async fn create_channel(&mut self, channel: Channel) -> Result<Channel, Box<dyn Error>> {
        let request = CreateChannelRequest {
            channel: Some(channel),
        };

        let res = self.client.create_channel(request).await?;

        Ok(res.into_inner())
    }

    pub async fn list_channel_members(
        &mut self,
        channel_id: ChannelId,
    ) -> Result<ListChannelMembersResponse, Box<dyn Error>> {
        let parent = format!("channels/{}", channel_id);

        let request = ListChannelMembersRequest {
            parent,
            pageable: None,
        };

        let res = self.client.list_channel_members(request).await?;

        Ok(res.into_inner())
    }

    pub async fn update_channel(&mut self, channel: Channel) -> Result<Channel, Box<dyn Error>> {
        let request = UpdateChannelRequest {
            channel: Some(channel),
        };

        let res = self.client.update_channel(request).await?;

        Ok(res.into_inner())
    }

    pub async fn delete_channel(&mut self, channel_id: ChannelId) -> Result<(), Box<dyn Error>> {
        let name = format!("channels/{}", channel_id);

        let request = DeleteChannelRequest { name };

        self.client.delete_channel(request).await?;

        Ok(())
    }

    pub async fn list_channel_messages(
        &mut self,
        channel_id: ChannelId,
    ) -> Result<(), Box<dyn Error>> {
        let name = format!("channels/{}", channel_id);
        let request = ListChannelMessagesRequest {
            name,
            pageable: None,
        };

        self.client.list_channel_messages(request).await?;

        Ok(())
    }

    pub async fn speech(
        &mut self,
        channel_id: ChannelId,
        content: String,
        reply_to: MessageId,
    ) -> Result<SpeechResponse, Box<dyn Error>> {
        let name = format!("channels/{}", channel_id);
        let reply_to = format!("messages/{}", reply_to);

        let request = SpeechRequest {
            name,
            content,
            reply_to,
        };

        let res = self.client.speech(request).await?;
        Ok(res.into_inner())
    }
}
