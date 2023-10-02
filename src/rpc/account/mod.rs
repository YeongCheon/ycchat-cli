use std::error::Error;
use std::sync::Arc;

use tokio::sync::Mutex;
use tonic::transport::Channel;
use tower::ServiceBuilder;

use super::interceptor::AuthMiddleware;
use super::ycchat_account::account_client::AccountClient;
use super::ycchat_account::{DeleteAccountRequest, UpdatePasswordRequest};
use super::ycchat_auth::SignInResponse;

pub struct AccountService {
    client: AccountClient<AuthMiddleware>,
}

impl AccountService {
    pub async fn new(auth_state: Arc<Mutex<SignInResponse>>) -> Result<Self, Box<dyn Error>> {
        let channel = Channel::from_static("http://127.0.0.1:50051")
            .connect()
            .await?;

        let auth_middleware = AuthMiddleware::new(channel.clone(), auth_state);

        let channel = ServiceBuilder::new().service(auth_middleware);

        let client = AccountClient::new(channel);

        Ok(Self { client })
    }

    pub async fn update_password(
        &mut self,
        current_password: String,
        new_password: String,
    ) -> Result<(), Box<dyn Error>> {
        let request = UpdatePasswordRequest {
            current_password,
            new_password,
        };

        self.client.update_password(request).await?;

        Ok(())
    }

    pub async fn delete_account(&mut self, reason: String) -> Result<(), Box<dyn Error>> {
        let request = DeleteAccountRequest { reason };

        self.client.delete_account(request).await?;

        Ok(())
    }
}
