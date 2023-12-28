use app_state::AppState;
use crossterm::event::{self, DisableMouseCapture, EnableMouseCapture};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, LeaveAlternateScreen};
use crossterm::{execute, terminal::EnterAlternateScreen};
use ratatui::prelude::{Backend, CrosstermBackend};
use ratatui::Terminal;
use std::cell::RefCell;
use std::sync::{Arc, Mutex};
use std::{error::Error, io};
use ui::after_sign_in::AfterSignInUi;
use ui::profile::ProfileUi;
use ui::sign_in::SignInUi;
use ui::sign_up::SignUpUi;
use ui::welcome::WelcomeUi;
use ui::{Scene, Ui};

mod app_state;
mod rpc;
mod ui;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;

    let mut stderr = io::stderr();

    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    let _ = run_app(&mut terminal).await;

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

async fn run_app<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<bool> {
    let app_state = AppState::new();
    let app_state = RefCell::new(app_state);
    let app_state = Arc::new(Mutex::new(app_state));

    let mut welcome_ui = WelcomeUi::new();
    let mut sign_in_ui = SignInUi::new(app_state.clone());
    let mut sign_up_ui = SignUpUi::new(app_state.clone());
    let mut after_sign_in_ui = AfterSignInUi::new(app_state.clone());
    let mut profile_ui = ProfileUi::new(app_state);

    let mut current_secene = Scene::Main;

    loop {
        let ui: &mut dyn Ui = match current_secene {
            Scene::Main => &mut welcome_ui,
            Scene::SignIn => &mut sign_in_ui,
            Scene::SignUp => &mut sign_up_ui,
            Scene::AfterSignIn => &mut after_sign_in_ui,
            Scene::Profile => &mut profile_ui,
            Scene::Quit => {
                return Ok(true);
            }
        };

        terminal.draw(|f| ui.ui(f))?;

        if event::poll(std::time::Duration::from_millis(50))? {
            let res = ui.event_handle(event::read()).await;

            if let Ok(scene) = res {
                current_secene = scene;

                match current_secene {
                    Scene::Quit => {
                        return Ok(true);
                    }
                    _ => {
                        continue;
                    }
                }
            }
        }
    }
}
