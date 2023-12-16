use tonic::transport::Channel;

use super::ycchat::v1::services::auth::auth_service_client::AuthServiceClient;
use super::ycchat::v1::services::auth::{
    SignInRequest, SignInResponse, SignUpRequest, SignUpResponse,
};

pub struct AuthService {
    client: AuthServiceClient<Channel>,
}

impl AuthService {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let client = AuthServiceClient::connect("http://127.0.0.1:50051").await?;
        Ok(Self { client })
    }

    pub async fn sign_in(
        &mut self,
        username: String,
        password: String,
    ) -> Result<SignInResponse, Box<dyn std::error::Error>> {
        let request = SignInRequest { username, password };

        let response = self.client.sign_in(request).await?;

        Ok(response.into_inner())
    }

    pub async fn sign_up(
        &mut self,
        email: String,
        username: String,
        password: String,
    ) -> Result<SignUpResponse, Box<dyn std::error::Error>> {
        let request = SignUpRequest {
            email,
            username,
            password,
        };

        let response = self.client.sign_up(request).await?;

        Ok(response.into_inner())
    }
}
