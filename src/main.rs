use std::{
    error::Error,
    io::{self, Write},
    str::FromStr,
};

use rpassword::read_password;
use rpc::{
    auth::AuthService,
    model::Server,
    server::ServerService,
    ycchat_auth::{SignInResponse, SignUpResponse},
};

mod rpc;

enum SignAction {
    SignUp,
    SignIn,
}

impl FromStr for SignAction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(Self::SignUp),
            "1" => Ok(Self::SignIn),
            _ => Err(()),
        }
    }
}

enum Step01Action {
    SelectServer,
    CreateServer,
    Exit,
}

impl FromStr for Step01Action {
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

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
    println!("Welcome!");

    let mut auth_service = AuthService::new().await?;

    let sign_in_response = sign_process(&mut auth_service).await?;

    let mut server_service = ServerService::new(sign_in_response).await?;

    println!("Choose Server Action");
    println!("[0]: Exit");
    println!("[1]: SelectServer");
    println!("[2]: CreateServer");

    loop {
        let mut server_action = String::new();
        let _ = io::stdin().read_line(&mut server_action);

        let action = match Step01Action::from_str(server_action.trim()) {
            Ok(res) => res,
            Err(_) => return Err("server action error".into()),
        };

        match action {
            Step01Action::SelectServer => {
                let list_server_response = server_service.list_server().await?;
                println!("server list size: {}", list_server_response.servers.len());

                list_server_response.servers.iter().for_each(|item| {
                    println!("{item:?}");
                });
            }
            Step01Action::CreateServer => {
                let display_name = read_line("input display_name: ");
                let description = read_line("input description: ");
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
            Step01Action::Exit => break,
        }
    }

    Ok(())
}

pub async fn sign_process(
    auth_service: &mut AuthService,
) -> Result<SignInResponse, Box<dyn Error>> {
    return loop {
        println!("[0]: sign up");
        println!("[1]: sign in");

        let mut sign_action = String::new();
        let _ = io::stdin().read_line(&mut sign_action);

        let sign_action = match SignAction::from_str(sign_action.trim()) {
            Ok(res) => res,
            Err(_) => continue,
        };

        match sign_action {
            SignAction::SignUp => {
                match sign_up(auth_service).await {
                    Ok(_) => {
                        println!("sign up success!");
                    }
                    Err(err) => {
                        eprintln!("{}", err);
                        continue;
                    }
                };
            }
            SignAction::SignIn => match sign_in(auth_service).await {
                Ok(res) => {
                    break Ok(res);
                }
                Err(err) => {
                    eprintln!("{}", err);
                    continue;
                }
            },
        };
    };
}

pub async fn sign_in(auth_service: &mut AuthService) -> Result<SignInResponse, Box<dyn Error>> {
    println!("\nSign In");

    let username = read_line("input username: ");
    let password = read_line("input password: ");

    let response = auth_service
        .sign_in(username.to_string(), password.to_string())
        .await?;

    Ok(response)
}

pub async fn sign_up(auth_service: &mut AuthService) -> Result<SignUpResponse, Box<dyn Error>> {
    println!("\nSign Up");

    let email = read_line("input email: ");
    let username = read_line("input username: ");
    let password = read_line("input password: ");
    let password_again = read_line("input password again: ");

    if password != password_again {
        return Result::Err("invalid password.".into());
    }

    let response = auth_service.sign_up(email, username, password).await?;

    Ok(response)
}

fn read_line(msg: &str) -> String {
    print!("{msg}");
    io::stdout().flush().unwrap();

    let mut buf = String::new();
    let _ = io::stdin().read_line(&mut buf);
    buf.trim().to_string()
}
