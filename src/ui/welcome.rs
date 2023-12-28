use std::{io, pin::Pin};

use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    prelude::{Color, Constraint, Direction, Layout},
    style::Style,
    text::{Line, Span, Text},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Frame,
};

use super::{Scene, Ui};

pub struct WelcomeUi<'a> {
    selected_index: usize,
    list_items: Vec<ListItem<'a>>,
}

impl<'a> WelcomeUi<'a> {
    pub fn new() -> Self {
        Self {
            selected_index: 0,
            list_items: vec![
                ListItem::new(Line::from(Span::styled("Sign In", Style::default()))),
                ListItem::new(Line::from(Span::styled("Sign Up", Style::default()))),
                ListItem::new(Line::from(Span::styled("Exit", Style::default()))),
            ],
        }
    }
}

impl<'a> Ui for WelcomeUi<'a> {
    fn ui(&self, f: &mut Frame) {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(3),
                Constraint::Percentage(100),
                // Constraint::Min(1),
            ])
            .split(f.size());

        let title_block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default());

        let title = Paragraph::new(Text::styled("YcChat", Style::default().fg(Color::Green)))
            .block(title_block);

        f.render_widget(title, layout[0]);

        let list_block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default());

        let mut state = ListState::default().with_selected(Some(self.selected_index));
        let list = List::new(self.list_items.clone())
            .block(list_block)
            .highlight_style(Style::default().bg(Color::LightCyan));

        f.render_stateful_widget(list, layout[1], &mut state);
    }

    fn event_handle<'me>(
        &'me mut self,
        event: std::io::Result<crossterm::event::Event>,
    ) -> Pin<Box<dyn std::future::Future<Output = io::Result<Scene>> + Send + 'me>> {
        let me: &'me mut WelcomeUi = self;

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
                            0 => return Ok(Scene::SignIn),

                            1 => return Ok(Scene::SignUp),

                            2 => {
                                return Ok(Scene::Quit);
                            }

                            _ => {
                                return Ok(Scene::Main);
                            }
                        },
                        KeyCode::Esc => {
                            return Ok(Scene::Quit);
                        }
                        _ => {}
                    }

                    Ok(Scene::Main)
                } else {
                    Ok(Scene::Main)
                }
            } else {
                Ok(Scene::Main)
            }
        })
    }
}
