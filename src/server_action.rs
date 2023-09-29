use std::{error::Error, io, str::FromStr};

use crate::rpc::{model::Server, server::ServerService, ycchat_auth::SignInResponse};

enum ServerAction {
    SelectServer,
    CreateServer,
    Exit,
}

impl FromStr for ServerAction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(Self::Exit),
            "1" => Ok(Self::SelectServer),
            "2" => Ok(Self::CreateServer),
            _ => Err(()),
        }
    }
}

pub async fn server_action(sign_in_response: SignInResponse) -> Result<(), Box<dyn Error>> {
    let mut server_service = ServerService::new(sign_in_response).await?;

    println!("Choose Server Action");
    println!("[0]: Exit");
    println!("[1]: SelectServer");
    println!("[2]: CreateServer");

    loop {
        let mut server_action = String::new();
        let _ = io::stdin().read_line(&mut server_action);

        let action = match ServerAction::from_str(server_action.trim()) {
            Ok(res) => res,
            Err(_) => return Err("server action error".into()),
        };

        match action {
            ServerAction::SelectServer => {
                let list_server_response = server_service.list_server().await?;
                println!("server list size: {}", list_server_response.servers.len());

                list_server_response.servers.iter().for_each(|item| {
                    println!("{item:?}");
                });
            }
            ServerAction::CreateServer => {
                let display_name = super::read_line("input display_name: ");
                let description = super::read_line("input description: ");
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
                println!("{created_server:?}");
            }
            ServerAction::Exit => return Ok(()),
        }
    }
}
