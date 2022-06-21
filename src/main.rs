use crossterm::{
    event::{
        self,
        DisableMouseCapture,
        EnableMouseCapture,
        Event,
        KeyCode
    },
    execute,
    terminal::{
        disable_raw_mode,
        enable_raw_mode,
        EnterAlternateScreen,
        LeaveAlternateScreen
    }
};
use std::{error::Error, io};
use tui::{
    backend::{Backend, CrosstermBackend},
    Terminal
};

mod app;
mod task_list;
mod ui;

use app::*;
use ui::*;

fn main() -> Result<(), Box<dyn Error>> {
    // Setup Terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    // Application Entry Point
    let mut app = App::new();
    let res = run_app(&mut terminal, &mut app);

    // Restore Terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    // Report Errors
    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<()> {
    loop {
        // Render UI
        terminal.draw(|frame| ui(frame, app))?;

        // Handle Events
        if let Event::Key(key) = event::read()? {
            match app.state {
                AppState::Tracker => {
                    match key.code {
                        KeyCode::Char('q') => return Ok(()),
                        KeyCode::Char('j') => app.list_down(),
                        KeyCode::Char('k') => app.list_up(),
                        KeyCode::Char('h') => app.prev_list(),
                        KeyCode::Char('l') => app.next_list(),
                        KeyCode::Char('c') => app.cycle_list_color(1),
                        KeyCode::Char('C') => app.cycle_list_color(-1),
                        KeyCode::Char(' ') => app.move_task_to_next_list(),
                        KeyCode::Backspace => app.move_task_to_prev_list(),
                        KeyCode::Enter => {
                            if !app.focused_list_is_empty() {
                                app.change_state(AppState::TaskView);
                            }
                        },
                        _ => {}
                    }
                },
                AppState::TaskView => {
                    match key.code {
                        KeyCode::Char('j') => app.scroll_details(1),
                        KeyCode::Char('k') => app.scroll_details(-1),
                        KeyCode::Char('q') => {
                            app.reset_scroll();
                            app.change_state(AppState::Tracker);
                        },
                        _ => {}
                    }
                },
            }
        }
    }
}
