pub mod auth;
pub mod server;
pub mod user;

mod interceptor;

pub mod model {
    tonic::include_proto!("ycchat.model");
}

pub mod ycchat_auth {
    tonic::include_proto!("ycchat.auth");
}

pub mod ycchat_user {
    tonic::include_proto!("ycchat.user");
}

pub mod ycchat_server {
    tonic::include_proto!("ycchat.server");
}
