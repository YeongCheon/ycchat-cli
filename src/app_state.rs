use crate::rpc::ycchat::v1::{models::User, services::auth::SignInResponse};

pub struct AppState {
    pub user: Option<UserState>,
}

impl AppState {
    pub fn new() -> Self {
        Self { user: None }
    }
}

#[derive(Clone)]
pub struct UserState {
    pub username: String,
    pub user: Option<User>,
    pub sign_in_response: SignInResponse,
}

impl UserState {
    pub fn new(username: String, user: Option<User>, sign_in_resposne: SignInResponse) -> Self {
        UserState {
            username,
            user,
            sign_in_response: sign_in_resposne,
        }
    }
}
