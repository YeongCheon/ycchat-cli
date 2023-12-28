use std::{
    cell::RefCell,
    error::Error,
    io,
    pin::Pin,
    sync::{Arc, Mutex},
};

use ratatui::{
    prelude::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::Text,
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use tui_textarea::{Input, Key, TextArea};

use crate::{
    app_state::{AppState, UserState},
    rpc::{auth::AuthService, me::MeUserService, ycchat::v1::services::auth::SignInResponse},
};

use super::{Scene, Ui};

pub struct SignInUi<'a> {
    app_state: Arc<Mutex<RefCell<AppState>>>,
    input_username: TextArea<'a>,
    input_password: TextArea<'a>,
    error_message: Option<String>,
    current_focus: Focus,
}

enum Focus {
    UserName,
    Password,
}

impl<'a> SignInUi<'a> {
    pub fn new(app_state: Arc<Mutex<RefCell<AppState>>>) -> Self {
        let mut input_username = TextArea::default();
        input_username.set_block(Block::default().borders(Borders::ALL).title("username"));
        input_username.set_placeholder_text("Please enter your username");

        let mut input_password = TextArea::default();
        input_password.set_block(Block::default().borders(Borders::ALL).title("password"));
        input_password.set_mask_char('\u{2022}'); //U+2022 BULLET (•)
        input_password.set_placeholder_text("Please enter your password"); //U+2022 BULLET (•)

        Self {
            app_state,
            input_username,
            input_password,
            error_message: None,
            current_focus: Focus::UserName,
        }
    }

    fn focus(&mut self, focus: Focus) {
        let enable_style = Style::default().bg(Color::White);
        let disable_style = Style::default();

        match focus {
            Focus::UserName => {
                self.input_username.set_cursor_style(enable_style);
                self.input_password.set_cursor_style(disable_style);
            }
            Focus::Password => {
                self.input_username.set_cursor_style(disable_style);
                self.input_password.set_cursor_style(enable_style);
            }
        }

        self.current_focus = focus;
    }

    async fn submit(&mut self) -> Result<SignInResponse, Box<dyn Error>> {
        let username = self.input_username.lines().join("").trim().to_string();
        let password = self.input_password.lines().join("").trim().to_string();

        let mut auth_service = AuthService::new().await?;
        let response = auth_service.sign_in(username.clone(), password).await?;

        {
            let mut me_user_service = {
                let auth_state = { Arc::new(tokio::sync::Mutex::new(response.clone())) };

                MeUserService::new(auth_state).await?
            };

            let user = if let Ok(user) = me_user_service.get_user().await {
                Some(user)
            } else {
                None
            };

            let mut app_state = self.app_state.lock().unwrap();
            let app_state = app_state.get_mut();
            app_state.user = Some(UserState::new(username, user, response.clone()));
        }

        Ok(response)
    }
}

impl<'a> Ui for SignInUi<'a> {
    fn ui(&self, f: &mut Frame) {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(3),
                Constraint::Min(3),
                Constraint::Min(3),
                Constraint::Min(3),
                // Constraint::Min(1),
            ])
            .split(f.size());

        let title_block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default());

        let title = Paragraph::new(Text::styled("Sign In", Style::default().fg(Color::Green)))
            .block(title_block);

        let err_message = match &self.error_message {
            Some(err_message) => err_message.clone(),
            None => String::new(),
        };

        f.render_widget(title, layout[0]);
        f.render_widget(self.input_username.widget(), layout[1]);
        f.render_widget(self.input_password.widget(), layout[2]);
        f.render_widget(
            Paragraph::new(Text::styled(
                err_message,
                Style::default().fg(Color::LightRed),
            )),
            layout[3],
        );
    }

    fn event_handle<'me>(
        &'me mut self,
        event: std::io::Result<crossterm::event::Event>,
    ) -> Pin<Box<dyn std::future::Future<Output = io::Result<Scene>> + Send + 'me>> {
        let me: &'me mut SignInUi = self;

        Box::pin(async {
            match event?.into() {
                Input { key: Key::Esc, .. } => return Ok(Scene::Main),
                Input { key: Key::Tab, .. } => {
                    let focus = match me.current_focus {
                        Focus::UserName => Focus::Password,
                        Focus::Password => Focus::UserName,
                    };

                    me.focus(focus);
                }
                Input {
                    key: Key::Enter, ..
                } => {
                    let sign_in_res = me.submit().await.unwrap();

                    me.error_message = Some(sign_in_res.user_id);

                    return Ok(Scene::AfterSignIn);
                }
                input => {
                    match me.current_focus {
                        Focus::UserName => me.input_username.input(input),
                        Focus::Password => me.input_password.input(input),
                    };
                }
            };

            Ok(Scene::SignIn)
        })
    }
}
