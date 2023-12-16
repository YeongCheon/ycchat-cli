use std::error::Error;
use std::sync::Arc;

use tokio::sync::Mutex;
use tonic::transport::Channel;
use tower::ServiceBuilder;

use super::interceptor::AuthMiddleware;
use super::server::ServerId;
use super::user::UserId;
use super::ycchat::v1::models::ServerMember;
use super::ycchat::v1::services::auth::SignInResponse;
use super::ycchat::v1::services::server::member::server_member_service_client::ServerMemberServiceClient;
use super::ycchat::v1::services::server::member::{
    GetServerMemberRequest, ListServerMembersRequest, ListServerMembersResponse,
};
// use super::ycchat_server_member::{
//     CreateServerRequest, DeleteServerRequest, GetServerRequest, LeaveServerRequest,
//     ListServersRequest, ListServersResponse, UpdateServerRequest,
// };

pub struct ServerMemberService {
    client: ServerMemberServiceClient<AuthMiddleware>,
}

impl ServerMemberService {
    pub async fn new(auth_state: Arc<Mutex<SignInResponse>>) -> Result<Self, Box<dyn Error>> {
        let channel = Channel::from_static("http://127.0.0.1:50051")
            .connect()
            .await?;

        let auth_middleware = AuthMiddleware::new(channel.clone(), auth_state);

        let channel = ServiceBuilder::new().service(auth_middleware);

        let client = ServerMemberServiceClient::new(channel);

        Ok(Self { client })
    }

    pub async fn list_server_members(
        &mut self,
        server_id: ServerId,
        page_size: i32,
        page_token: Option<String>,
    ) -> Result<ListServerMembersResponse, Box<dyn Error>> {
        let parent = format!("servers/{server_id}");

        let request = ListServerMembersRequest {
            parent,
            page_size,
            page_token,
        };

        let response = self.client.list_server_members(request).await?;

        Ok(response.into_inner())
    }

    pub async fn get_server_member(
        &mut self,
        server_id: ServerId,
        user_id: UserId,
    ) -> Result<ServerMember, Box<dyn Error>> {
        let name = format!("/servers/{server_id}/members/{user_id}");
        let request = GetServerMemberRequest { name };

        let response = self.client.get_server_member(request).await?;

        Ok(response.into_inner())
    }
}
