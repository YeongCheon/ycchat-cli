use std::error::Error;

use tonic::service::interceptor::InterceptedService;
use tonic::transport::Channel;

use super::interceptor::AuthInterceptor;
use super::ycchat_account::account_client::AccountClient;
use super::ycchat_account::{DeleteAccountRequest, UpdatePasswordRequest};
use super::ycchat_auth::SignInResponse;

pub struct AccountService {
    client: AccountClient<InterceptedService<Channel, AuthInterceptor>>,
}

impl AccountService {
    pub async fn new(sign_in_res: SignInResponse) -> Result<Self, Box<dyn Error>> {
        let channel = Channel::from_static("http://127.0.0.1:50051")
            .connect()
            .await?;

        let client = AccountClient::with_interceptor(channel, AuthInterceptor::new(sign_in_res));

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
