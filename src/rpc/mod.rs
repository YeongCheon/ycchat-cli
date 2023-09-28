pub mod auth;
mod interceptor;
pub mod server;

pub mod model {
    tonic::include_proto!("ycchat.model");
}

pub mod ycchat_auth {
    tonic::include_proto!("ycchat.auth");
}

pub mod ycchat_server {
    tonic::include_proto!("ycchat.server");
}
