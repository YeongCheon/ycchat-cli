pub mod account;
pub mod auth;
pub mod category;
pub mod channel;
pub mod connect;
pub mod me;
pub mod message;
pub mod server;
pub mod server_member;
pub mod user;

mod interceptor;

pub mod ycchat {
    pub mod v1 {
        pub mod models {
            tonic::include_proto!("ycchat.v1.models");
        }

        pub mod services {
            pub mod auth {
                tonic::include_proto!("ycchat.v1.services.auth");
            }

            pub mod account {
                tonic::include_proto!("ycchat.v1.services.account");
            }

            pub mod channel {
                tonic::include_proto!("ycchat.v1.services.channel");
            }

            pub mod connect {
                tonic::include_proto!("ycchat.v1.services.connect");
            }

            pub mod me {
                pub mod user {
                    tonic::include_proto!("ycchat.v1.services.me.user");
                }
            }

            pub mod user {
                tonic::include_proto!("ycchat.v1.services.user");
            }

            pub mod server {
                tonic::include_proto!("ycchat.v1.services.server");

                pub mod category {
                    tonic::include_proto!("ycchat.v1.services.server.category");
                }

                pub mod member {
                    tonic::include_proto!("ycchat.v1.services.server.member");
                }
            }

            pub mod message {
                tonic::include_proto!("ycchat.v1.services.message");
            }
        }
    }
}
