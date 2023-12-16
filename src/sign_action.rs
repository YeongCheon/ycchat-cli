use std::{error::Error, fmt::Display};

use enum_iterator::{all, Sequence};
use inquire::{required, Password, Select, Text};

use crate::rpc::{
    auth::AuthService,
    ycchat::v1::services::auth::{SignInResponse, SignUpResponse},
};

#[derive(Debug, PartialEq, Sequence)]
enum SignAction {
    SignUp,
    SignIn,
    Exit,
}

pub enum ActionResult {
    SignUp(SignUpResponse),
    SignIn(SignInResponse),
    Exit,
}

impl Display for SignAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            SignAction::SignUp => "SignUp",
            SignAction::SignIn => "SignIn",
            SignAction::Exit => "Exit",
        };

        write!(f, "{}", text)
    }
}

pub async fn action() -> Result<ActionResult, Box<dyn Error>> {
    let mut auth_service = AuthService::new().await?;

    let action = {
        let items = all::<SignAction>().collect();
        Select::new("SignAction:", items).prompt()?
    };

    let sign_in_res = match action {
        SignAction::SignUp => {
            let res = sign_up(&mut auth_service).await?;
            ActionResult::SignUp(res)
        }
        SignAction::SignIn => ActionResult::SignIn(sign_in(&mut auth_service).await?),
        SignAction::Exit => ActionResult::Exit,
    };

    Ok(sign_in_res)
}

async fn sign_up(auth_service: &mut AuthService) -> Result<SignUpResponse, Box<dyn Error>> {
    println!("\nSign Up");

    let email = Text::new("Email:")
        .with_validator(required!("This field is required"))
        .with_help_message("e.g. kyc1682@gmail.com")
        .prompt()?;

    let username = Text::new("Username:")
        .with_validator(required!("This field is required"))
        .with_help_message("e.g. kyc1682")
        .prompt()?;

    let password = Password::new("Password:").prompt()?;

    let response = auth_service.sign_up(email, username, password).await?;

    Ok(response)
}

async fn sign_in(auth_service: &mut AuthService) -> Result<SignInResponse, Box<dyn Error>> {
    println!("\nSign In");

    let username = Text::new("Username:")
        .with_validator(required!("This field is required"))
        .with_help_message("e.g. kyc1682")
        .prompt()?;

    let password = Password::new("Password:").without_confirmation().prompt()?;

    let response = auth_service
        .sign_in(username.to_string(), password.to_string())
        .await?;

    Ok(response)
}
