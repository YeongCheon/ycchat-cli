use std::error::Error;
use std::sync::Arc;

use tokio::sync::Mutex;
use tonic::transport::Channel;
use tower::ServiceBuilder;
use ulid::Ulid;

use crate::rpc::ycchat_server::EnterServerRequest;

use super::interceptor::AuthMiddleware;
use super::model::{Attachment, Server, ServerMember};
use super::ycchat_auth::SignInResponse;
use super::ycchat_server::server_client::ServerClient;
use super::ycchat_server::{
    CreateServerRequest, DeleteServerRequest, GetServerRequest, LeaveServerRequest,
    ListServersRequest, ListServersResponse, UpdateServerRequest,
};

pub type ServerId = Ulid;

pub struct ServerService {
    client: ServerClient<AuthMiddleware>,
}

impl ServerService {
    pub async fn new(auth_state: Arc<Mutex<SignInResponse>>) -> Result<Self, Box<dyn Error>> {
        let channel = Channel::from_static("http://127.0.0.1:50051")
            .connect()
            .await?;

        let auth_middleware = AuthMiddleware::new(channel.clone(), auth_state);

        let channel = ServiceBuilder::new().service(auth_middleware);

        let client = ServerClient::new(channel);

        Ok(Self { client })
    }

    pub async fn create_server(&mut self, server: Server) -> Result<Server, Box<dyn Error>> {
        let request = CreateServerRequest {
            server: Some(server),
        };

        let response = self.client.create_server(request).await?;

        Ok(response.into_inner())
    }

    pub async fn list_server(&mut self) -> Result<ListServersResponse, Box<dyn Error>> {
        let request = ListServersRequest { pageable: None };

        let response = self.client.list_servers(request).await?;

        Ok(response.into_inner())
    }

    pub async fn get_server(&mut self, server_id: ServerId) -> Result<Server, Box<dyn Error>> {
        let name = format!("servers/{}", server_id);
        let request = GetServerRequest { name };

        let response = self.client.get_server(request).await?;

        Ok(response.into_inner())
    }

    pub async fn update_server(&mut self, server: Server) -> Result<Server, Box<dyn Error>> {
        let request = UpdateServerRequest {
            server: Some(server),
        };

        let response = self.client.update_server(request).await?;

        Ok(response.into_inner())
    }

    pub async fn delete_server(&mut self, server_id: ServerId) -> Result<(), Box<dyn Error>> {
        let name = format!("servers/{}", server_id);

        let request = DeleteServerRequest { name };

        self.client.delete_server(request).await?;

        Ok(())
    }

    pub async fn enter_server(
        &mut self,
        server_id: ServerId,
        display_name: String,
        description: String,
        avartar: Option<Attachment>,
    ) -> Result<ServerMember, Box<dyn Error>> {
        let name = format!("servers/{}", server_id);

        let request = EnterServerRequest {
            name,
            display_name,
            description,
            avartar,
        };

        let response = self.client.enter_server(request).await?;

        Ok(response.into_inner())
    }

    pub async fn leave_server(&mut self, server_id: ServerId) -> Result<(), Box<dyn Error>> {
        let name = format!("servers/{}", server_id);

        let request = LeaveServerRequest { name };

        self.client.leave_server(request).await?;

        Ok(())
    }
}
