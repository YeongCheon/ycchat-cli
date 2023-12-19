use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    prelude::{Color, Constraint, Direction, Layout},
    style::Style,
    text::{Line, Span, Text},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Frame,
};

use super::{Scene, Ui};

pub struct MainUi<'a> {
    selected_index: usize,
    list_items: Vec<ListItem<'a>>,
}

impl<'a> MainUi<'a> {
    pub fn new() -> Self {
        Self {
            selected_index: 0,
            list_items: vec![
                ListItem::new(Line::from(Span::styled("Sign In", Style::default()))),
                ListItem::new(Line::from(Span::styled("Sign Up", Style::default()))),
                ListItem::new(Line::from(Span::styled("Exit ('q')", Style::default()))),
            ],
        }
    }
}

impl<'a> Ui for MainUi<'a> {
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

    fn event_handle(
        &mut self,
        event: std::io::Result<crossterm::event::Event>,
    ) -> std::io::Result<Scene> {
        if let Event::Key(key) = event? {
            if key.kind == event::KeyEventKind::Press {
                match key.code {
                    KeyCode::Up => {
                        if self.selected_index > 0 {
                            self.selected_index -= 1;
                        }
                    }
                    KeyCode::Down => {
                        if self.selected_index < self.list_items.len() - 1 {
                            self.selected_index += 1;
                        }
                    }
                    KeyCode::Enter => {
                        if self.selected_index == 2 {
                            return Ok(Scene::Quit);
                        }
                    }
                    KeyCode::Char('q') => {
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
    }
}
