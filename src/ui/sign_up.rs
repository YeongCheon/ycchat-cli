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
    app_state::AppState,
    rpc::{auth::AuthService, ycchat::v1::services::auth::SignUpResponse},
};

use super::{Scene, Ui};

pub struct SignUpUi<'a> {
    app_state: Arc<Mutex<RefCell<AppState>>>,

    input_username: TextArea<'a>,
    input_email: TextArea<'a>,
    input_password: TextArea<'a>,
    input_password_repeat: TextArea<'a>,

    current_focus: Focus,
}

enum Focus {
    UserName,
    Email,
    Password,
    PasswordRepeat,
}

impl<'a> SignUpUi<'a> {
    pub fn new(app_state: Arc<Mutex<RefCell<AppState>>>) -> Self {
        let mut input_username = TextArea::default();
        input_username.set_block(Block::default().borders(Borders::ALL).title("username"));
        input_username.set_placeholder_text("Please enter your username");

        let mut input_email = TextArea::default();
        input_email.set_block(Block::default().borders(Borders::ALL).title("email"));
        input_email.set_placeholder_text("Please enter your email");

        let mut input_password = TextArea::default();
        input_password.set_block(Block::default().borders(Borders::ALL).title("password"));
        input_password.set_mask_char('\u{2022}'); //U+2022 BULLET (•)
        input_password.set_placeholder_text("Please enter your password"); //U+2022 BULLET (•)

        let mut input_password_repeat = TextArea::default();
        input_password_repeat.set_block(
            Block::default()
                .borders(Borders::ALL)
                .title("password repeat"),
        );
        input_password_repeat.set_mask_char('\u{2022}'); //U+2022 BULLET (•)
        input_password_repeat.set_placeholder_text("Please enter your password again."); //U+2022 BULLET (•)

        Self {
            app_state,
            input_username,
            input_email,
            input_password,
            input_password_repeat,
            current_focus: Focus::UserName,
        }
    }

    fn focus(&mut self, focus: Focus) {
        let enable_style = Style::default().bg(Color::White);

        self.disable_all_cursor_style();
        match focus {
            Focus::UserName => {
                self.input_username.set_cursor_style(enable_style);
            }
            Focus::Email => {
                self.input_email.set_cursor_style(enable_style);
            }
            Focus::Password => {
                self.input_password.set_cursor_style(enable_style);
            }
            Focus::PasswordRepeat => {
                self.input_password_repeat.set_cursor_style(enable_style);
            }
        }

        self.current_focus = focus;
    }

    async fn submit(&self) -> Result<SignUpResponse, Box<dyn Error>> {
        let email = self.input_email.lines().join("").trim().to_string();
        let username = self.input_username.lines().join("").trim().to_string();
        let password = self.input_password.lines().join("").trim().to_string();

        let mut auth_service = AuthService::new().await?;
        let response = auth_service.sign_up(email, username, password).await?;

        Ok(response)
    }

    fn disable_all_cursor_style(&mut self) {
        let disable_style = Style::default();

        self.input_username.set_cursor_style(disable_style);
        self.input_email.set_cursor_style(disable_style);
        self.input_password.set_cursor_style(disable_style);
        self.input_password_repeat.set_cursor_style(disable_style);
    }
}

impl<'a> Ui for SignUpUi<'a> {
    fn ui(&self, f: &mut Frame) {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(3), // title
                Constraint::Min(3), // username
                Constraint::Min(3), // email
                Constraint::Min(3), // password
                Constraint::Min(3), // password repeat
                Constraint::Min(3), // error message
            ])
            .split(f.size());

        let title_block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default());

        let title = Paragraph::new(Text::styled("Sign Up", Style::default().fg(Color::Green)))
            .block(title_block);

        f.render_widget(title, layout[0]);
        f.render_widget(self.input_username.widget(), layout[1]);
        f.render_widget(self.input_email.widget(), layout[2]);
        f.render_widget(self.input_password.widget(), layout[3]);
        f.render_widget(self.input_password_repeat.widget(), layout[4]);
    }

    fn event_handle<'me>(
        &'me mut self,
        event: std::io::Result<crossterm::event::Event>,
    ) -> Pin<Box<dyn std::future::Future<Output = io::Result<Scene>> + Send + 'me>> {
        let me: &'me mut SignUpUi = self;

        Box::pin(async move {
            match event?.into() {
                Input { key: Key::Esc, .. } => return Ok(Scene::Main),
                Input { key: Key::Tab, .. } => {
                    let focus = match me.current_focus {
                        Focus::UserName => Focus::Email,
                        Focus::Email => Focus::Password,
                        Focus::Password => Focus::PasswordRepeat,
                        Focus::PasswordRepeat => Focus::UserName,
                    };

                    me.focus(focus);
                }
                Input {
                    key: Key::Enter, ..
                } => {
                    let _ = me.submit().await;

                    return Ok(Scene::Main);
                }
                input => {
                    match me.current_focus {
                        Focus::UserName => me.input_username.input(input),
                        Focus::Email => me.input_email.input(input),
                        Focus::Password => me.input_password.input(input),
                        Focus::PasswordRepeat => me.input_password_repeat.input(input),
                    };
                }
            };

            Ok(Scene::SignUp)
        })
    }
}
