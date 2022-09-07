use crossterm::{
    event::{
        DisableMouseCapture,
        EnableMouseCapture,
        KeyboardEnhancementFlags,
        PushKeyboardEnhancementFlags,
        PopKeyboardEnhancementFlags
    },
    execute,
    terminal::{
        disable_raw_mode,
        enable_raw_mode,
        EnterAlternateScreen,
        LeaveAlternateScreen
    }
};
use std::{
    env,
    error::Error,
    fs,
    io,
    path::PathBuf,
    process
};
use tui::{
    backend::{Backend, CrosstermBackend},
    Terminal
};

mod app;
mod events;
mod inputs;
mod lists;
mod ui;

use app::*;
use events::*;
use ui::*;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let args = get_command_line_args();
    setup_project_path()?;

    // Panic Handling
    chain_hook();

    // Setup Terminal
    let mut terminal = init_terminal()?;
    terminal.clear()?;

    // Application Entry Point
    let res: io::Result<()>;
    if project_exists(&args[1])? {
        let mut app = App::create(args[1].clone())?;
        res = run_app(&mut terminal, &mut app);
    } else {
        res = Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("Project '{}' does not exist.\nTo create it, run 'kadai' with no arguments, then press 'n'.", &args[1])
            )
        );
    }


    // Restore Terminal
    terminal.show_cursor()?;
    reset_terminal()?;

    // Report Errors
    if let Err(err) = res {
        println!("{}", err.into_inner().unwrap());
    }

    Ok(())
}

fn get_command_line_args() -> Vec<String> {
    let mut args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        // If no project is given, push an empty name into args
        args.push(String::new());
    }

    args
}

fn setup_project_path() -> Result<()> {
    let path = get_user_home()?;
    get_kadai_directory(path)?;

    Ok(())
}

fn get_user_home() -> Result<PathBuf> {
    let user_home = env::var("HOME");
    let user_home = user_home.unwrap_or_else(|_| {
        eprintln!("Could not find environment variable: $HOME");
        process::exit(1);
    });
    let path = PathBuf::from(&user_home);
    env::set_current_dir(&path)?;

    Ok(path)
}

fn get_kadai_directory(path: PathBuf) -> Result<PathBuf> {
    let path = path.join(".kadai");
    if !path.exists() {
        fs::create_dir(&path)?;
    }
    env::set_current_dir(&path)?;

    Ok(path)
}

fn project_exists(project: &String) -> Result<bool> {
    let mut path = env::current_dir()?;
    path.push(project);

    Ok(path.exists())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<()> {
    loop {
        if app.quit {
            break;
        }

        terminal.draw(|frame| ui(frame, app, app.state.clone()))?;
        handle_events(app)?;
    }

    app.save_changes()?;

    Ok(())
}

pub fn chain_hook() {
    let original_hook = std::panic::take_hook();

    std::panic::set_hook(Box::new(move |panic| {
        reset_terminal().unwrap();
        original_hook(panic);
    }));
}

pub fn init_terminal() -> Result<Terminal<CrosstermBackend<io::Stdout>>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(
        stdout,
        EnterAlternateScreen,
        EnableMouseCapture,
        PushKeyboardEnhancementFlags(
            KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES
        )
    )?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;

    Ok(terminal)
}

pub fn reset_terminal() -> Result<()> {
    disable_raw_mode()?;
    execute!(
        io::stdout(),
        LeaveAlternateScreen,
        DisableMouseCapture,
        PopKeyboardEnhancementFlags
    )?;

    Ok(())
}
