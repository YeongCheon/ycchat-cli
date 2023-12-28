use std::{
    cell::RefCell,
    sync::{Arc, Mutex},
};

use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    prelude::Direction,
    prelude::{Constraint, Layout},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
};

use crate::app_state::AppState;

use super::{Scene, Ui};

pub struct AfterSignInUi<'a> {
    app_state: Arc<Mutex<RefCell<AppState>>>,
    selected_index: usize,
    list_items: Vec<ListItem<'a>>,
}

impl<'a> AfterSignInUi<'a> {
    pub fn new(app_state: Arc<Mutex<RefCell<AppState>>>) -> Self {
        AfterSignInUi {
            app_state,
            selected_index: 0,
            list_items: vec![
                ListItem::new(Line::from(Span::styled("Chat", Style::default()))),
                ListItem::new(Line::from(Span::styled("Profile", Style::default()))),
                ListItem::new(Line::from(Span::styled("Sign out", Style::default()))),
            ],
        }
    }

    fn sign_out(&mut self) {
        let mut app_state = self.app_state.lock().unwrap();
        app_state.get_mut().user = None;
    }
}

impl<'a> Ui for AfterSignInUi<'a> {
    fn ui(&self, f: &mut ratatui::Frame) {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(3), Constraint::Percentage(100)])
            .split(f.size());

        let title = {
            let username: String = {
                let app_state = self.app_state.lock().unwrap();
                let app_state = app_state.borrow();

                match &app_state.user {
                    Some(user_state) => match &user_state.user {
                        Some(user) => user.display_name.clone(),
                        None => user_state.username.clone(),
                    },
                    None => "unknown".to_string(),
                }
            };

            Paragraph::new(Text::styled(
                format!("Welcome {}", username),
                Style::default().fg(Color::Green),
            ))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .style(Style::default()),
            )
        };

        f.render_widget(title, layout[0]);

        let list_block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default());

        {
            let mut list_state = ListState::default().with_selected(Some(self.selected_index));
            let list = List::new(self.list_items.clone())
                .block(list_block)
                .highlight_style(Style::default().bg(Color::LightCyan));

            f.render_stateful_widget(list, layout[1], &mut list_state);
        }
    }

    fn event_handle<'me>(
        &'me mut self,
        event: std::io::Result<crossterm::event::Event>,
    ) -> std::pin::Pin<
        Box<dyn std::future::Future<Output = std::io::Result<super::Scene>> + Send + 'me>,
    > {
        let me: &'me mut Self = self;

        Box::pin(async move {
            if let Event::Key(key) = event? {
                if key.kind == event::KeyEventKind::Press {
                    match key.code {
                        KeyCode::Up => {
                            if me.selected_index > 0 {
                                me.selected_index -= 1;
                            }
                        }
                        KeyCode::Down => {
                            if me.selected_index < me.list_items.len() - 1 {
                                me.selected_index += 1;
                            }
                        }
                        KeyCode::Enter => match me.selected_index {
                            // 0 => return Ok(Scene::SignIn),
                            1 => return Ok(Scene::Profile),
                            2 => {
                                me.sign_out();
                                return Ok(Scene::Main);
                            }

                            _ => {
                                return Ok(Scene::AfterSignIn);
                            }
                        },
                        // KeyCode::Esc => {
                        //     return Ok(Scene::Quit);
                        // }
                        _ => {}
                    }

                    Ok(Scene::AfterSignIn)
                } else {
                    Ok(Scene::AfterSignIn)
                }
            } else {
                Ok(Scene::AfterSignIn)
            }
        })
    }
}
