use std::error::Error;
use std::sync::Arc;

use tokio::sync::Mutex;
use tonic::transport::Channel;
use tower::ServiceBuilder;
use ulid::Ulid;

use super::interceptor::AuthMiddleware;
use super::model::User;
use super::ycchat_auth::SignInResponse;
use super::ycchat_user::user_client::UserClient;
use super::ycchat_user::{CreateUserRequest, DeleteUserRequest, GetUserRequest, UpdateUserRequest};

pub type UserId = Ulid;

pub struct UserService {
    client: UserClient<AuthMiddleware>,
}

impl UserService {
    pub async fn new(auth_state: Arc<Mutex<SignInResponse>>) -> Result<Self, Box<dyn Error>> {
        let channel = Channel::from_static("http://127.0.0.1:50051")
            .connect()
            .await?;

        let auth_middleware = AuthMiddleware::new(channel.clone(), auth_state);

        let channel = ServiceBuilder::new().service(auth_middleware);

        let client = UserClient::new(channel);

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
