use std::{
    error::Error,
    io::{self, Write},
    str::FromStr,
};

use rpassword::read_password;
use rpc::{
    auth::AuthService,
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
                let list = server_service.list_server().await?;
            }
            Step01Action::CreateServer => {
                let created_server = server_service.create_server().await?;
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

    let username = {
        print!("input username: ");
        io::stdout().flush().unwrap();

        let mut username = String::new();
        let _ = io::stdin().read_line(&mut username);
        username.trim().to_string()
    };

    let password = {
        print!("input password: ");
        io::stdout().flush().unwrap();

        let password: String = read_password().unwrap();
        password.trim().to_string()
    };

    let response = auth_service
        .sign_in(username.to_string(), password.to_string())
        .await?;

    Ok(response)
}

pub async fn sign_up(auth_service: &mut AuthService) -> Result<SignUpResponse, Box<dyn Error>> {
    println!("\nSign Up");

    let email: String = {
        print!("input email: ");
        io::stdout().flush().unwrap();

        let mut email = String::new();
        let _ = io::stdin().read_line(&mut email);
        email.trim().to_string()
    };

    let username: String = {
        print!("input username: ");
        io::stdout().flush().unwrap();

        let mut username = String::new();
        let _ = io::stdin().read_line(&mut username);
        username.trim().to_string()
    };

    let password: String = {
        print!("input password: ");
        io::stdout().flush().unwrap();

        let password: String = read_password().unwrap();
        password.trim().to_string()
    };

    let password_again: String = {
        print!("input password again: ");
        io::stdout().flush().unwrap();

        let password_again: String = read_password().unwrap();
        password_again.trim().to_string()
    };

    if password != password_again {
        return Result::Err("invalid password.".into());
    }

    let response = auth_service.sign_up(email, username, password).await?;

    Ok(response)
}
