use enum_iterator::{all, Sequence};
use inquire::Select;
use rpassword::read_password;
use rpc::{
    auth::AuthService,
    ycchat_auth::{SignInResponse, SignUpResponse},
};
use std::{
    error::Error,
    io::{self, Write},
    str::FromStr,
};
use terminal_menu::{button, label, menu, mut_menu, run};

mod rpc;
mod server_action;
mod user_action;

enum SignAction {
    SignUp,
    SignIn,
}

impl FromStr for SignAction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "sign up" => Ok(Self::SignUp),
            "sign in" => Ok(Self::SignIn),
            _ => Err(()),
        }
    }
}

#[derive(Debug, PartialEq, Sequence)]
enum Action {
    Account,
    User,
    Server,
}

impl Action {
    fn value(&self) -> &str {
        match self {
            Action::Account => "account",
            Action::User => "user",
            Action::Server => "server",
        }
    }
}

impl FromStr for Action {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "account" => Ok(Self::Account),
            "user" => Ok(Self::User),
            "server" => Ok(Self::Server),
            _ => Err(()),
        }
    }
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
    println!("Welcome!");

    let mut auth_service = AuthService::new().await?;

    let sign_in_response = sign_process(&mut auth_service).await?;

    let action = {
        let mut items = vec![
            // label:
            //  not selectable, useful as a title, separator, etc...
            label("----------------------"),
            label("terminal-menu"),
            label("use wasd or arrow keys"),
            label("enter to select"),
            label("'q' or esc to exit"),
            label("-----------------------"),
        ];

        all::<Action>().for_each(|action| {
            let item = button(action.value());
            items.push(item);
        });

        let menu = menu(items);
        run(&menu);

        // reference1: https://users.rust-lang.org/t/creates-a-temporary-which-is-freed-while-still-in-use-again/29211/2?u=yeongcheon
        // reference2: https://www.christopherbiscardi.com/rust-creates-a-temporary-which-is-freed-while-still-in-use
        let selected_item = mut_menu(&menu);
        let selected_item = selected_item.selected_item_name();
        let selected_item = selected_item.trim();

        match Action::from_str(selected_item) {
            Ok(action) => action,
            Err(_) => return Err("parse action error".into()),
        }
    };

    match action {
        Action::Account => todo!(),
        Action::User => todo!(),
        Action::Server => {
            server_action::server_action(sign_in_response).await?;
        }
    }

    Ok(())
}

pub async fn sign_process(
    auth_service: &mut AuthService,
) -> Result<SignInResponse, Box<dyn Error>> {
    return loop {
        let sign_action = Select::new("Sign:", vec!["sign up", "sign in"]).prompt()?;

        // let mut sign_action = String::new();
        // let _ = io::stdin().read_line(&mut sign_action);

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

    let email = read_line("input email: ");
    let username = read_line("input username: ");
    let password = {
        print!("input password: ");
        io::stdout().flush().unwrap();

        let password: String = read_password().unwrap();
        password.trim().to_string()
    };
    let password_again = {
        print!("input password again: ");
        io::stdout().flush().unwrap();

        let password: String = read_password().unwrap();
        password.trim().to_string()
    };

    if password != password_again {
        return Result::Err("invalid password.".into());
    }

    let response = auth_service.sign_up(email, username, password).await?;

    Ok(response)
}

pub fn read_line(msg: &str) -> String {
    print!("{msg}");
    io::stdout().flush().unwrap();

    let mut buf = String::new();
    let _ = io::stdin().read_line(&mut buf);
    buf.trim().to_string()
}
