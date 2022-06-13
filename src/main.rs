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
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    text::Span,
    widgets::{Block, Borders},
    Frame,
    Terminal
};

struct App;

impl App {
    fn new() -> Self {
        Self
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // Setup Terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Application Entry Point
    let app = App::new();
    let res = run_app(&mut terminal, app);

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

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: App) -> io::Result<()> {
    loop {
        // Render UI
        terminal.draw(|frame| ui(frame, &app))?;

        // Handle Events
        if let Event::Key(key) = event::read()? {
            if let KeyCode::Char('q') = key.code {
                return Ok(());
            }
        }
    }
}

fn ui<B: Backend>(frame: &mut Frame<B>, _app: &App) {
    let size = frame.size();
    let third = size.width / 3;

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Length(third),
                Constraint::Min(10),
                Constraint::Length(third),
            ]
            .as_ref()
        )
        .split(size);

    let block = Block::default()
        .title(
            Span::styled(
                "Planned",
                Style::default().fg(Color::Magenta)
            )
        )
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL);
    frame.render_widget(block, chunks[0]);

    let block = Block::default()
        .title(
            Span::styled(
                "In Progess",
                Style::default().fg(Color::Yellow)
            )
        )
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL);
    frame.render_widget(block, chunks[1]);

    let block = Block::default()
        .title(
            Span::styled(
                "Completed",
                Style::default().fg(Color::Green)
            )
        )
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL);
    frame.render_widget(block, chunks[2]);
}
