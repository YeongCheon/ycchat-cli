use std::{error::Error, str::FromStr};

use crate::rpc::{model::User, user::UserService, ycchat_auth::SignInResponse};
use enum_iterator::{all, Sequence};
use terminal_menu::{button, label, menu, mut_menu, run};
use ulid::Ulid;

#[derive(Debug, PartialEq, Sequence)]
enum UserAction {
    Get,
    Create,
    Update,
    Delete,
}

impl UserAction {
    fn value(&self) -> &str {
        match self {
            UserAction::Get => "get",
            UserAction::Create => "create",
            UserAction::Update => "update",
            UserAction::Delete => "delete",
        }
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
            _ => Err(()),
        }
    }
}

pub async fn action(sign_in_response: SignInResponse) -> Result<(), Box<dyn Error>> {
    let mut user_service = UserService::new(sign_in_response).await?;

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

        all::<UserAction>().for_each(|action| {
            let item = button(action.value());
            items.push(item);
        });

        let menu = menu(items);
        run(&menu);

        let selected_item = mut_menu(&menu);
        let selected_item = selected_item.selected_item_name();
        let selected_item = selected_item.trim();

        match UserAction::from_str(selected_item) {
            Ok(action) => action,
            Err(_) => return Err("parse action error".into()),
        }
    };

    match action {
        UserAction::Get => {
            let user_id = Ulid::new(); // FIXME
            user_service.get_user(user_id).await?;
        }
        UserAction::Create => {
            let user_id = Ulid::new(); // FIXME
            let name = format!("user/{}", user_id);

            let display_name = "display_name".to_string(); // FIXME
            let description = "description".to_string(); // FIXME
            let avatar = None;
            let region_code = "3".to_string();
            let language_code = "ko-KR".to_string();
            let time_zone = "Asia/Seoul".to_string();

            let user = User {
                name,
                display_name,
                description,
                avatar,
                region_code: Some(region_code),
                language_code: Some(language_code),
                time_zone: Some(time_zone),
                create_time: None,
                update_time: None,
            };

            user_service.create_user(user).await?;
        }
        UserAction::Update => {
            let user_id = Ulid::new(); // FIXME
            let name = format!("user/{}", user_id);

            let display_name = "display_name".to_string(); // FIXME
            let description = "description".to_string(); // FIXME
            let avatar = None;
            let region_code = "3".to_string();
            let language_code = "ko-KR".to_string();
            let time_zone = "Asia/Seoul".to_string();

            let user = User {
                name,
                display_name,
                description,
                avatar,
                region_code: Some(region_code),
                language_code: Some(language_code),
                time_zone: Some(time_zone),
                create_time: None,
                update_time: None,
            };

            user_service.update_user(user).await?;
        }
        UserAction::Delete => {
            let user_id = Ulid::new(); // FIXME
            user_service.delete_user(user_id).await?;
        }
    }

    Ok(())
}
