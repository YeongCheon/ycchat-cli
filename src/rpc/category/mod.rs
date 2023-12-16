use std::error::Error;
use std::sync::Arc;

use super::interceptor::AuthMiddleware;
use super::server::ServerId;
use super::ycchat::v1::models::Category;
use super::ycchat::v1::services::auth::SignInResponse;
use super::ycchat::v1::services::server::category::category_service_client::CategoryServiceClient;
use super::ycchat::v1::services::server::category::{
    CreateCategoryRequest, DeleteCategoryRequest, GetCategoryRequest, GetCategoryResponse,
    ListCategoriesRequest, ListCategoriesResponse, UpdateCategoryRequest,
};
use tokio::sync::Mutex;
use tonic::transport::Channel;
use tower::ServiceBuilder;
use ulid::Ulid;

pub type CategoryId = Ulid;

pub struct CategoryService {
    client: CategoryServiceClient<AuthMiddleware>,
}

impl CategoryService {
    pub async fn new(auth_state: Arc<Mutex<SignInResponse>>) -> Result<Self, Box<dyn Error>> {
        let channel = Channel::from_static("http://127.0.0.1:50051")
            .connect()
            .await?;

        let auth_middleware = AuthMiddleware::new(channel.clone(), auth_state);

        let channel = ServiceBuilder::new().service(auth_middleware);

        let client = CategoryServiceClient::new(channel);

        Ok(Self { client })
    }

    pub async fn list_categories(
        &mut self,
        server_id: ServerId,
        page_size: i32,
        page_token: Option<String>,
    ) -> Result<ListCategoriesResponse, Box<dyn Error>> {
        let parent = format!("servers/{server_id}");

        let request = ListCategoriesRequest {
            parent,
            page_size,
            page_token,
        };

        let response = self.client.list_categories(request).await?;

        Ok(response.into_inner())
    }

    pub async fn get_category(&mut self) -> Result<GetCategoryResponse, Box<dyn Error>> {
        let name = format!("");
        let request = GetCategoryRequest { name };

        let response = self.client.get_category(request).await?;

        Ok(response.into_inner())
    }

    pub async fn create_category(
        &mut self,
        category: Category,
    ) -> Result<Category, Box<dyn Error>> {
        let parent = format!("");
        let category_id = format!("");

        let request = CreateCategoryRequest {
            parent,
            category_id,
            category: Some(category),
        };

        let response = self.client.create_category(request).await?;

        Ok(response.into_inner())
    }

    pub async fn update_category(
        &mut self,
        category: Category,
    ) -> Result<Category, Box<dyn Error>> {
        let request = UpdateCategoryRequest {
            category: Some(category),
        };

        let response = self.client.update_category(request).await?;

        Ok(response.into_inner())
    }

    pub async fn delete_category(&mut self, category_id: CategoryId) -> Result<(), Box<dyn Error>> {
        let name = format!("{}", category_id);

        let request = DeleteCategoryRequest { name };

        let response = self.client.delete_category(request).await?;

        Ok(())
    }
}
