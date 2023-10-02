use std::{error::Error, fmt::Display, str::FromStr};

use crate::rpc::{
    model::User,
    user::{UserId, UserService},
    ycchat_auth::SignInResponse,
};
use enum_iterator::{all, Sequence};
use inquire::{Select, Text};

#[derive(Debug, PartialEq, Sequence)]
enum UserAction {
    Get,
    Create,
    Update,
    Delete,
    Exit,
}

impl Display for UserAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            UserAction::Get => "get",
            UserAction::Create => "create",
            UserAction::Update => "update",
            UserAction::Delete => "delete",
            UserAction::Exit => "exit",
        };

        write!(f, "{}", text)
    }
}

impl FromStr for UserAction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "get" => Ok(Self::Get),
            "create" => Ok(Self::Create),
            "update" => Ok(Self::Update),
            "delete" => Ok(Self::Delete),
            "exit" => Ok(Self::Exit),
            _ => Err(()),
        }
    }
}

pub async fn action(sign_in_response: SignInResponse) -> Result<(), Box<dyn Error>> {
    let user_id = UserId::from_string(&sign_in_response.user_id)?;
    let mut user_service = UserService::new(sign_in_response).await?;

    let action = {
        let items = all::<UserAction>().collect();

        Select::new("UserAction:", items).prompt()?
    };

    match action {
        UserAction::Get => {
            let res = user_service.get_user(user_id).await?;

            println!("{:#?}", res);
        }
        UserAction::Create => {
            let name = format!("user/{}", user_id);

            let avatar = None;

            let display_name = Text::new("input display_name:").prompt()?;
            let description = Text::new("input description:").prompt()?;
            let region_code = Select::new("select region_code:", vec!["KR", "US"]).prompt()?;
            let language_code =
                Select::new("select language_code:", vec!["ko-KR", "en-US"]).prompt()?;
            let time_zone = Select::new("select time_zone:", vec!["Asia/Seoul"]).prompt()?;

            let user = User {
                name,
                display_name,
                description,
                avatar,
                region_code: Some(region_code.to_string()),
                language_code: Some(language_code.to_string()),
                time_zone: Some(time_zone.to_string()),
                create_time: None,
                update_time: None,
            };

            let res = user_service.create_user(user).await?;

            println!("{:#?}", res);
        }
        UserAction::Update => {
            let name = format!("user/{}", user_id);

            let display_name = Text::new("input display_name:").prompt()?;
            let description = Text::new("input description:").prompt()?;
            let region_code = Select::new("select region_code:", vec!["1", "2", "3"]).prompt()?;
            let language_code =
                Select::new("select language_code:", vec!["ko-KR", "en-US"]).prompt()?;
            let time_zone = Select::new("select time_zone:", vec!["Asia/Seoul"]).prompt()?;

            let avatar = None;

            let user = User {
                name,
                display_name,
                description,
                avatar,
                region_code: Some(region_code.to_string()),
                language_code: Some(language_code.to_string()),
                time_zone: Some(time_zone.to_string()),
                create_time: None,
                update_time: None,
            };

            let res = user_service.update_user(user).await?;

            println!("{:#?}", res);
        }
        UserAction::Delete => {
            user_service.delete_user(user_id).await?;

            println!("delete complete");
        }

        UserAction::Exit => return Ok(()),
    }

    Ok(())
}
