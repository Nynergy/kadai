use crossterm::{
    event::{
        DisableMouseCapture,
        EnableMouseCapture
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
    io::{self, Write},
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
mod task_list;
mod ui;

use app::*;
use events::*;
use ui::*;

fn main() -> Result<(), Box<dyn Error>> {
    let args = get_command_line_args();
    setup_project_path(&args)?;

    // Setup Terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    // Application Entry Point
    let mut app = App::create(args[1].clone())?;
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

fn get_command_line_args() -> Vec<String> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("USAGE: {} <project-name>", &args[0]);
        process::exit(0);
    }

    args
}

fn setup_project_path(args: &Vec<String>) -> Result<(), io::Error> {
    let path = get_user_home()?;
    let path = get_kadai_directory(path)?;
    get_project_directory(path, args)?;

    Ok(())
}

fn get_user_home() -> Result<PathBuf, io::Error> {
    let user_home = env::var("HOME");
    let user_home = user_home.unwrap_or_else(|_| {
        eprintln!("Could not find environment variable: $HOME");
        process::exit(1);
    });
    let path = PathBuf::from(&user_home);
    env::set_current_dir(&path)?;

    Ok(path)
}

fn get_kadai_directory(path: PathBuf) -> Result<PathBuf, io::Error> {
    let path = path.join(".kadai");
    if !path.exists() {
        fs::create_dir(&path)?;
    }
    env::set_current_dir(&path)?;

    Ok(path)
}

fn get_project_directory(path: PathBuf, args: &Vec<String>) -> Result<(), io::Error> {
    let path = path.join(&args[1]);
    if !path.exists() {
        confirm_project_creation(&path, args)?;
    }
    env::set_current_dir(&path)?;

    Ok(())
}

fn confirm_project_creation(path: &PathBuf, args: &Vec<String>) -> Result<(), io::Error> {
    let mut input = String::new();
    print!("Project '{}' does not exist. Create it? [y/N] ", &args[1]);
    io::stdout().flush()?;
    io::stdin().read_line(&mut input).expect("Did not enter a correct string");
    if let Some('\n') = input.chars().next_back() {
        input.pop();
    }
    if let Some('\r') = input.chars().next_back() {
        input.pop();
    }

    if input == "y" || input == "Y" {
        fs::create_dir(&path)?;
    } else {
        println!("Project not created, exiting...");
        process::exit(0);
    }

    Ok(())
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
