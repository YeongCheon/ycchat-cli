use std::{error::Error, fmt::Display, sync::Arc};

use enum_iterator::{all, Sequence};
use inquire::{Password, Select, Text};
use tokio::sync::Mutex;

use crate::rpc::{account::AccountService, ycchat_auth::SignInResponse};

#[derive(Debug, PartialEq, Sequence)]
enum AccountAction {
    UpdatePassword,
    DeleteAccount,
    Exit,
}

impl Display for AccountAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            AccountAction::UpdatePassword => "UpdatePassword",
            AccountAction::DeleteAccount => "DeleteAccount",
            AccountAction::Exit => "Exit",
        };

        write!(f, "{}", text)
    }
}

pub async fn action(auth_state: Arc<Mutex<SignInResponse>>) -> Result<(), Box<dyn Error>> {
    let mut account_service = AccountService::new(auth_state).await?;

    loop {
        let action = {
            let items = all::<AccountAction>().collect();
            Select::new("ServerAction:", items).prompt()?
        };

        match action {
            AccountAction::UpdatePassword => {
                let current_password = Password::new("Current Password:")
                    .without_confirmation()
                    .prompt()?;

                let new_password = Password::new("New Password").prompt()?;

                account_service
                    .update_password(current_password, new_password)
                    .await?;
            }
            AccountAction::DeleteAccount => {
                let reason = { Text::new("reason:").prompt()? };

                account_service.delete_account(reason).await?;
            }
            AccountAction::Exit => break Ok(()),
        }
    }
}
