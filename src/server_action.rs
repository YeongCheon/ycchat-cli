use std::{error::Error, fmt::Display, sync::Arc};

use enum_iterator::{all, Sequence};
use inquire::{Select, Text};
use tokio::sync::Mutex;

use crate::rpc::{model::Server, server::ServerService, ycchat_auth::SignInResponse};

#[derive(Debug, PartialEq, Sequence)]
enum ServerAction {
    List,
    Create,
    Exit,
}

impl Display for ServerAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            ServerAction::List => "List",
            ServerAction::Create => "Create",
            ServerAction::Exit => "Exit",
        };

        write!(f, "{}", text)
    }
}

pub async fn server_action(auth_state: Arc<Mutex<SignInResponse>>) -> Result<(), Box<dyn Error>> {
    let mut server_service = ServerService::new(auth_state).await?;

    loop {
        let action = {
            let items = all::<ServerAction>().collect();
            Select::new("ServerAction:", items).prompt()?
        };

        match action {
            ServerAction::List => {
                let list_server_response = server_service.list_server().await?;
                println!("server list size: {}", list_server_response.servers.len());

                list_server_response.servers.iter().for_each(|item| {
                    println!("{item:#?}");
                });
            }
            ServerAction::Create => {
                let display_name = Text::new("input display_name:").prompt()?;
                let description = Text::new("input description:").prompt()?;
                let icon = None;
                let categories = vec![];
                let channels = vec![];

                let server = Server {
                    name: String::new(),
                    display_name,
                    description,
                    icon,
                    categories,
                    channels,
                    create_time: None,
                    update_time: None,
                };

                let created_server = server_service.create_server(server).await?;
                println!("{created_server:#?}");
            }
            ServerAction::Exit => return Ok(()),
        }
    }
}
