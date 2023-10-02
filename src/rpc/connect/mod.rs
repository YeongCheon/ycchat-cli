use std::error::Error;
use std::sync::Arc;

use super::interceptor::AuthMiddleware;
use super::ycchat_auth::SignInResponse;
use super::ycchat_connect::connect_client::ConnectClient;
use super::ycchat_connect::{ConnectRequest, ConnectResponse};

use tokio::sync::Mutex;
use tonic::transport::Channel;
use tonic::Streaming;
use tower::ServiceBuilder;

struct ConnectService {
    client: ConnectClient<AuthMiddleware>,
}

impl ConnectService {
    pub async fn new(auth_state: Arc<Mutex<SignInResponse>>) -> Result<Self, Box<dyn Error>> {
        let channel = Channel::from_static("http://127.0.0.1:50051")
            .connect()
            .await?;

        let auth_middleware = AuthMiddleware::new(channel.clone(), auth_state);

        let channel = ServiceBuilder::new().service(auth_middleware);

        let client = ConnectClient::new(channel);

        Ok(Self { client })
    }

    pub async fn connect(&mut self) -> Result<Streaming<ConnectResponse>, Box<dyn Error>> {
        let request = ConnectRequest {};
        let res = self.client.conn(request).await?;

        Ok(res.into_inner())
    }
}
