use std::error::Error;

use tonic::service::interceptor::InterceptedService;
use tonic::transport::Channel;

use super::interceptor::AuthInterceptor;
use super::model::Server;
use super::ycchat_auth::SignInResponse;
use super::ycchat_server::server_client::ServerClient;
use super::ycchat_server::{CreateServerRequest, ListServersRequest, ListServersResponse};

pub struct ServerService {
    client: ServerClient<InterceptedService<Channel, AuthInterceptor>>,
}

impl ServerService {
    pub async fn new(sign_in_res: SignInResponse) -> Result<Self, Box<dyn Error>> {
        let channel = Channel::from_static("http://127.0.0.1:50051")
            .connect()
            .await?;

        let client = ServerClient::with_interceptor(channel, AuthInterceptor::new(sign_in_res));

        Ok(Self { client })
    }

    pub async fn create_server(&mut self) -> Result<Server, Box<dyn Error>> {
        let request = CreateServerRequest {
            server: None, // FIXME
        };

        let response = self.client.create_server(request).await?;

        Ok(response.into_inner())
    }

    pub async fn list_server(&mut self) -> Result<ListServersResponse, Box<dyn Error>> {
        let request = ListServersRequest { pageable: None };

        let response = self.client.list_servers(request).await?;

        Ok(response.into_inner())
    }
}
