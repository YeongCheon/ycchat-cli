use std::{
    cell::RefCell,
    error::Error,
    sync::{Arc, Mutex},
};

use ratatui::{
    prelude::Direction,
    prelude::{Constraint, Layout},
    style::{Color, Style},
    text::Text,
    widgets::{Block, Borders, Paragraph},
};
use tui_textarea::{Input, Key, TextArea};

use crate::{
    app_state::AppState,
    rpc::{me::MeUserService, user::UserService, ycchat::v1::models::User},
};

use super::{Scene, Ui};

pub struct ProfileUi<'a> {
    current_focus: Focus,
    app_state: Arc<Mutex<RefCell<AppState>>>,
    input_display_name: TextArea<'a>,
    input_description: TextArea<'a>,
}

enum Focus {
    DisplayName,
    Description,
}

impl<'a> ProfileUi<'a> {
    pub fn new(app_state: Arc<Mutex<RefCell<AppState>>>) -> Self {
        let enable_style = Style::default().bg(Color::White);
        let disable_style = Style::default();

        let enable_block = Block::default().borders(Borders::ALL);
        let disable_block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::DarkGray));

        let mut input_display_name = TextArea::default();
        input_display_name.set_cursor_style(enable_style);
        input_display_name.set_block(enable_block.title("display name"));
        input_display_name.set_placeholder_text("Please enter your display name");

        let mut input_description = TextArea::default();
        input_description.set_cursor_style(disable_style);
        input_description.set_block(disable_block.title("description"));
        input_description.set_placeholder_text("Please enter description.");

        Self {
            current_focus: Focus::DisplayName,
            app_state,
            input_display_name,
            input_description,
        }
    }

    fn focus(&mut self, focus: Focus) {
        let enable_style = Style::default().bg(Color::White);
        let disable_style = Style::default();

        let enable_block = Block::default().borders(Borders::ALL);
        let disable_block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::DarkGray));

        match focus {
            Focus::DisplayName => {
                self.input_display_name.set_cursor_style(enable_style);
                self.input_display_name
                    .set_block(enable_block.title("display name"));

                self.input_description.set_cursor_style(disable_style);
                self.input_description
                    .set_block(disable_block.title("description"));
            }
            Focus::Description => {
                self.input_display_name.set_cursor_style(disable_style);
                self.input_display_name
                    .set_block(disable_block.title("display name"));

                self.input_description.set_cursor_style(enable_style);
                self.input_description
                    .set_block(enable_block.title("description"));
            }
        }

        self.current_focus = focus;
    }

    async fn submit(&mut self) -> Result<User, Box<dyn Error>> {
        let display_name = self.input_display_name.lines().join("").trim().to_string();
        let description = self.input_description.lines().join("").trim().to_string();

        let (sign_in_response, user) = {
            let app_state = self.app_state.lock().unwrap();
            let app_state = &app_state.borrow();

            let user_state = if let Some(user_state) = &app_state.user {
                user_state
            } else {
                return Err("invalid user state".into());
            };

            let sign_in_response = user_state.sign_in_response.clone();
            let user = if let Some(user) = &user_state.user {
                let mut user = user.clone();
                user.display_name = display_name;
                user.description = description;

                user
            } else {
                let user_id = user_state.sign_in_response.user_id.clone();
                let name = format!("user/{}", user_id);

                let avatar = None;
                let region_code = "1";
                let language_code = "ko-KR";
                let time_zone = "Asia/Seoul";

                User {
                    name,
                    display_name,
                    description,
                    avatar,
                    region_code: Some(region_code.to_string()),
                    language_code: Some(language_code.to_string()),
                    time_zone: Some(time_zone.to_string()),
                    create_time: None,
                    update_time: None,
                }
            };

            (sign_in_response, user)
        };

        let mut me_user_service = {
            let auth_state = { Arc::new(tokio::sync::Mutex::new(sign_in_response.clone())) };

            MeUserService::new(auth_state).await?
        };

        let mut user_service = {
            let auth_state = { Arc::new(tokio::sync::Mutex::new(sign_in_response.clone())) };

            UserService::new(auth_state).await?
        };

        let is_user_exist = me_user_service.get_user().await.is_ok();

        let res = if is_user_exist {
            user_service.update_user(user).await?
        } else {
            user_service.create_user(user).await?
        };

        {
            let app_state = self.app_state.lock().unwrap();
            let app_state = &mut app_state.borrow_mut();

            if let Some(user_state) = &mut app_state.user {
                user_state.user = Some(res.clone());
            }
        }

        Ok(res)
    }
}

impl<'a> Ui for ProfileUi<'a> {
    fn ui(&self, f: &mut ratatui::Frame) {
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

        let title = Paragraph::new(Text::styled("Profile", Style::default().fg(Color::Green)))
            .block(title_block);

        f.render_widget(title, layout[0]);

        f.render_widget(self.input_display_name.widget(), layout[1]);
        f.render_widget(self.input_description.widget(), layout[2]);
    }

    fn event_handle<'me>(
        &'me mut self,
        event: std::io::Result<crossterm::event::Event>,
    ) -> std::pin::Pin<
        Box<dyn std::future::Future<Output = std::io::Result<super::Scene>> + Send + 'me>,
    > {
        let me: &'me mut Self = self;

        Box::pin(async {
            match event?.into() {
                Input { key: Key::Esc, .. } => return Ok(Scene::Main),
                Input { key: Key::Tab, .. } => {
                    let focus = match me.current_focus {
                        Focus::DisplayName => Focus::Description,
                        Focus::Description => Focus::DisplayName,
                    };

                    me.focus(focus);
                }
                Input {
                    key: Key::Enter, ..
                } => {
                    let res = me.submit().await;
                    if let Ok(res) = res {
                        return Ok(Scene::AfterSignIn);
                    } else {
                        return Ok(Scene::SignIn);
                    }
                }
                input => {
                    match me.current_focus {
                        Focus::DisplayName => me.input_display_name.input(input),
                        Focus::Description => me.input_description.input(input),
                    };
                }
            };

            Ok(Scene::Profile)
        })
    }
}
