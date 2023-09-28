use std::{
    error::Error,
    io::{self, Write},
    str::FromStr,
};

use rpassword::read_password;
use rpc::{
    auth::AuthService,
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

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
    println!("Welcome!");

    let sign_in_response = loop {
        println!("[0]: sign up");
        println!("[1]: sign in");

        let mut sign_action = String::new();
        let _ = io::stdin().read_line(&mut sign_action);

        let sign_action = match SignAction::from_str(sign_action.trim()) {
            Ok(res) => res,
            Err(_) => continue,
        };

        let mut auth_service = AuthService::new().await?;

        match sign_action {
            SignAction::SignUp => {
                match sign_up(&mut auth_service).await {
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
                    break res;
                }
                Err(err) => {
                    eprintln!("{}", err);
                    continue;
                }
            },
        };
    };

    Ok(())
}

pub async fn sign_in(mut auth_service: AuthService) -> Result<SignInResponse, Box<dyn Error>> {
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
