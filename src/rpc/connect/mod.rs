use std::error::Error;

use super::interceptor::AuthInterceptor;
use super::ycchat_auth::SignInResponse;
use super::ycchat_connect::connect_client::ConnectClient;
use super::ycchat_connect::{ConnectRequest, ConnectResponse};

use tonic::service::interceptor::InterceptedService;
use tonic::transport::Channel;
use tonic::Streaming;

struct ConnectService {
    client: ConnectClient<InterceptedService<Channel, AuthInterceptor>>,
}

impl ConnectService {
    pub async fn new(sign_in_res: SignInResponse) -> Result<Self, Box<dyn Error>> {
        let channel = Channel::from_static("http://127.0.0.1:50051")
            .connect()
            .await?;

        let client = ConnectClient::with_interceptor(channel, AuthInterceptor::new(sign_in_res));

        Ok(Self { client })
    }

    pub async fn connect(&mut self) -> Result<Streaming<ConnectResponse>, Box<dyn Error>> {
        let request = ConnectRequest {};
        let res = self.client.conn(request).await?;

        Ok(res.into_inner())
    }
}
