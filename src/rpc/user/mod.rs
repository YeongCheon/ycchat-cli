use std::error::Error;

use tonic::service::interceptor::InterceptedService;
use tonic::transport::Channel;
use ulid::Ulid;

use super::interceptor::AuthInterceptor;
use super::model::User;
use super::ycchat_auth::SignInResponse;
use super::ycchat_user::user_client::UserClient;
use super::ycchat_user::{CreateUserRequest, DeleteUserRequest, GetUserRequest, UpdateUserRequest};

pub type UserId = Ulid;

pub struct UserService {
    client: UserClient<InterceptedService<Channel, AuthInterceptor>>,
}

impl UserService {
    pub async fn new(sign_in_res: SignInResponse) -> Result<Self, Box<dyn Error>> {
        let channel = Channel::from_static("http://127.0.0.1:50051")
            .connect()
            .await?;

        let client = UserClient::with_interceptor(channel, AuthInterceptor::new(sign_in_res));

        Ok(Self { client })
    }

    pub async fn get_user(&mut self, user_id: UserId) -> Result<User, Box<dyn Error>> {
        let name = format!("users/{}", user_id);
        let request = GetUserRequest { name };

        let response = self.client.get_user(request).await?;

        Ok(response.into_inner())
    }

    pub async fn create_user(&mut self, user: User) -> Result<User, Box<dyn Error>> {
        let request = CreateUserRequest { user: Some(user) };

        let response = self.client.create_user(request).await?;

        Ok(response.into_inner())
    }

    pub async fn update_user(&mut self, user: User) -> Result<User, Box<dyn Error>> {
        let request = UpdateUserRequest { user: Some(user) };

        let response = self.client.update_user(request).await?;

        Ok(response.into_inner())
    }

    // FIXME: user_id to ulid
    pub async fn delete_user(&mut self, user_id: UserId) -> Result<(), Box<dyn Error>> {
        let name = format!("users/{}", user_id);

        let request = DeleteUserRequest { name };

        self.client.delete_user(request).await?;

        Ok(())
    }
}
