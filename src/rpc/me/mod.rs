use std::{error::Error, sync::Arc};

use tokio::sync::Mutex;
use tonic::transport::Channel;
use tower::ServiceBuilder;

use super::{
    interceptor::AuthMiddleware,
    user::UserId,
    ycchat::v1::{
        models::User,
        services::{
            auth::SignInResponse,
            me::user::{me_user_service_client::MeUserServiceClient, GetMeRequest},
        },
    },
};

pub struct MeUserService {
    client: MeUserServiceClient<AuthMiddleware>,
}

impl MeUserService {
    pub async fn new(auth_state: Arc<Mutex<SignInResponse>>) -> Result<Self, Box<dyn Error>> {
        let channel = Channel::from_static("http://127.0.0.1:50051")
            .connect()
            .await?;

        let auth_middleware = AuthMiddleware::new(channel.clone(), auth_state);

        let channel = ServiceBuilder::new().service(auth_middleware);

        let client = MeUserServiceClient::new(channel);

        Ok(Self { client })
    }

    pub async fn get_user(&mut self) -> Result<User, Box<dyn Error>> {
        let request = GetMeRequest {};
        let response = self.client.get_me(request).await?;

        Ok(response.into_inner())
    }
}
