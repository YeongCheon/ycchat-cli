use std::error::Error;

use tonic::service::interceptor::InterceptedService;
use tonic::transport::Channel;

use super::interceptor::AuthInterceptor;
use super::model::ServerMember;
use super::server::ServerId;
use super::user::UserId;
use super::ycchat_auth::SignInResponse;
use super::ycchat_server::member::server_member_client::ServerMemberClient;
use super::ycchat_server::member::{
    GetServerMemberRequest, ListServerMembersRequest, ListServerMembersResponse,
};
// use super::ycchat_server_member::{
//     CreateServerRequest, DeleteServerRequest, GetServerRequest, LeaveServerRequest,
//     ListServersRequest, ListServersResponse, UpdateServerRequest,
// };

pub struct ServerMemberService {
    client: ServerMemberClient<InterceptedService<Channel, AuthInterceptor>>,
}

impl ServerMemberService {
    pub async fn new(sign_in_res: SignInResponse) -> Result<Self, Box<dyn Error>> {
        let channel = Channel::from_static("http://127.0.0.1:50051")
            .connect()
            .await?;

        let client =
            ServerMemberClient::with_interceptor(channel, AuthInterceptor::new(sign_in_res));

        Ok(Self { client })
    }

    pub async fn list_server_members(
        &mut self,
        server_id: ServerId,
    ) -> Result<ListServerMembersResponse, Box<dyn Error>> {
        let request = ListServerMembersRequest { pageable: None };
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
