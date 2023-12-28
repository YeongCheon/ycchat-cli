use std::{io, pin::Pin};

use crossterm::event::Event;
use ratatui::Frame;

pub mod after_sign_in;
pub mod profile;
pub mod sign_in;
pub mod sign_up;
pub mod welcome;

pub trait Ui {
    fn ui(&self, f: &mut Frame);

    fn event_handle<'me>(
        &'me mut self,
        event: std::io::Result<Event>,
    ) -> Pin<Box<dyn std::future::Future<Output = io::Result<Scene>> + Send + 'me>>;
}

pub enum Scene {
    Main,
    SignIn,
    SignUp,
    AfterSignIn,
    Profile,
    Quit, // close app
}
