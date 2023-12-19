use std::io;

use crossterm::event::Event;
use ratatui::Frame;

use self::main::MainUi;

pub mod main;

pub trait Ui {
    fn ui(&self, f: &mut Frame);
    fn event_handle(&mut self, event: std::io::Result<Event>) -> io::Result<Scene>;
}

pub enum Scene {
    Main,
    Quit, // close app
}
