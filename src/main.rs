use enum_iterator::{all, Sequence};
use inquire::Select;
use rpc::ycchat_auth::SignInResponse;
use std::{error::Error, fmt::Display, sync::Arc};
use tokio::sync::Mutex;

mod account_action;
mod rpc;
mod server_action;
mod sign_action;
mod user_action;

#[derive(Debug, PartialEq, Sequence)]
enum Action {
    Account,
    User,
    Server,
    Exit,
}

impl Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            Action::Account => "account",
            Action::User => "user",
            Action::Server => "server",
            Action::Exit => "exit",
        };

        write!(f, "{}", text)
    }
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
    println!("Welcome!");

    let sign_in_response = sign_process().await?;
    let auth_state = Arc::new(Mutex::new(sign_in_response));

    loop {
        let action = {
            let items = all::<Action>().collect();

            // reference1: https://users.rust-lang.org/t/creates-a-temporary-which-is-freed-while-still-in-use-again/29211/2
            // reference2: https://www.christopherbiscardi.com/rust-creates-a-temporary-which-is-freed-while-still-in-use

            Select::new("Action:", items).prompt()?
        };

        match action {
            Action::Account => account_action::action(auth_state.clone()).await?,
            Action::User => user_action::action(auth_state.clone()).await?,
            Action::Server => server_action::server_action(auth_state.clone()).await?,
            Action::Exit => break,
        }
    }

    Ok(())
}

pub async fn sign_process() -> Result<SignInResponse, Box<dyn Error>> {
    loop {
        let res = sign_action::action().await?;

        match res {
            sign_action::ActionResult::SignUp(_) => continue,
            sign_action::ActionResult::SignIn(sign_in_res) => break Ok(sign_in_res),
            sign_action::ActionResult::Exit => continue,
        }
    }
}
