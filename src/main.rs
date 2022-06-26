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
use std::{error::Error, env, fs, io::{self, Write}, path::PathBuf, process};
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
    // Command Line Arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("USAGE: {} <project-name>", &args[0]);
        return Ok(());
    }

    // Environment Variables
    let user_home = env::var("HOME");
    let user_home = user_home.unwrap_or_else(|_| {
        eprintln!("Could not find environment variable: $HOME");
        process::exit(1);
    });
    let mut path = PathBuf::from(&user_home);
    env::set_current_dir(&path)?;

    // Check if ~/.kadai exists, and if it doesn't, create it
    path.push(".kadai");
    if !path.exists() {
        fs::create_dir(&path)?;
    }
    env::set_current_dir(&path)?;

    // Check if project dir exists, and if it doesn't, create it
    // TODO: Make project name optional and present project select screen
    path.push(&args[1]);
    if !path.exists() {
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
    }
    env::set_current_dir(&path)?;

    // Setup Terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    // Application Entry Point
    let mut app = App::create()?;
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
        terminal.draw(|frame| ui(frame, app, app.state.clone()))?;

        // Handle Events
        if let Event::Key(key) = event::read()? {
            let state = app.state.clone();
            match state {
                AppState::Tracker => {
                    match key.code {
                        KeyCode::Char('q') => break,
                        KeyCode::Char('n') => {
                            app.clear_detail_inputs();
                            app.reset_active_detail_input();
                            app.change_state(AppState::CreateTask(Box::new(state)));
                        },
                        KeyCode::Char('d') => {
                            if !app.focused_list_is_empty() {
                                app.change_state(AppState::DeleteTask(Box::new(state)));
                            }
                        },
                        KeyCode::Char('e') => {
                            if !app.focused_list_is_empty() {
                                app.populate_task_detail_inputs();
                                app.reset_active_detail_input();
                                app.change_state(AppState::EditTask(Box::new(state)));
                            }
                        },
                        KeyCode::Char('N') => {
                            app.clear_list_inputs();
                            app.change_state(AppState::CreateList(Box::new(state)));
                        },
                        KeyCode::Char('D') => {
                            app.change_state(AppState::DeleteList(Box::new(state)));
                        },
                        KeyCode::Char('E') => {
                            app.populate_list_detail_inputs();
                            app.change_state(AppState::EditList(Box::new(state)));
                        },
                        KeyCode::Char('j') => app.list_down(),
                        KeyCode::Char('k') => app.list_up(),
                        KeyCode::Char('h') => app.prev_list(),
                        KeyCode::Char('l') => app.next_list(),
                        KeyCode::Char('J') => app.task_down(),
                        KeyCode::Char('K') => app.task_up(),
                        KeyCode::Char('H') => app.list_left(),
                        KeyCode::Char('L') => app.list_right(),
                        KeyCode::Char('c') => app.cycle_list_color(1),
                        KeyCode::Char('C') => app.cycle_list_color(-1),
                        KeyCode::Char(' ') => app.move_task_to_next_list(),
                        KeyCode::Backspace => app.move_task_to_prev_list(),
                        KeyCode::Enter => {
                            if !app.focused_list_is_empty() {
                                app.change_state(AppState::TaskView(Box::new(state)));
                            }
                        },
                        KeyCode::Char('b') => app.change_state(AppState::BacklogPopup(Box::new(state))),
                        KeyCode::Char('B') => app.move_task_to_backlog(),
                        KeyCode::Char('a') => app.change_state(AppState::ArchivePopup(Box::new(state))),
                        KeyCode::Char('A') => app.move_task_to_archive(),
                        _ => {}
                    }
                },
                AppState::TaskView(prev) => {
                    match key.code {
                        KeyCode::Char('q') => break,
                        KeyCode::Char('j') => app.scroll_details(1),
                        KeyCode::Char('k') => app.scroll_details(-1),
                        KeyCode::Enter => {
                            app.reset_scroll();
                            app.change_state(*prev);
                        },
                        _ => {}
                    }
                },
                AppState::BacklogPopup(prev) => {
                    match key.code {
                        KeyCode::Char('q') => break,
                        KeyCode::Char('n') => {
                            app.clear_detail_inputs();
                            app.reset_active_detail_input();
                            app.change_state(AppState::CreateTask(Box::new(AppState::BacklogPopup(prev))));
                        },
                        KeyCode::Char('d') => {
                            if !app.focused_list_is_empty() {
                                app.change_state(AppState::DeleteTask(Box::new(AppState::BacklogPopup(prev))));
                            }
                        },
                        KeyCode::Char('e') => {
                            if !app.focused_list_is_empty() {
                                app.populate_task_detail_inputs();
                                app.reset_active_detail_input();
                                app.change_state(AppState::EditTask(Box::new(AppState::BacklogPopup(prev))));
                            }
                        },
                        KeyCode::Char('j') => app.list_down(),
                        KeyCode::Char('k') => app.list_up(),
                        KeyCode::Char('J') => app.task_down(),
                        KeyCode::Char('K') => app.task_up(),
                        KeyCode::Char(' ') => app.move_task_to_list(0),
                        KeyCode::Char('c') => app.cycle_list_color(1),
                        KeyCode::Char('C') => app.cycle_list_color(-1),
                        KeyCode::Char('b') => app.change_state(*prev),
                        KeyCode::Char('a') => app.change_state(AppState::ArchivePopup(prev)),
                        KeyCode::Enter => {
                            if !app.focused_list_is_empty() {
                                app.change_state(AppState::TaskView(Box::new(AppState::BacklogPopup(prev))));
                            }
                        },
                        _ => {}
                    }
                },
                AppState::ArchivePopup(prev) => {
                    match key.code {
                        KeyCode::Char('q') => break,
                        KeyCode::Char('d') => {
                            if !app.focused_list_is_empty() {
                                app.change_state(AppState::DeleteTask(Box::new(AppState::ArchivePopup(prev))));
                            }
                        },
                        KeyCode::Char('j') => app.list_down(),
                        KeyCode::Char('k') => app.list_up(),
                        KeyCode::Char('J') => app.task_down(),
                        KeyCode::Char('K') => app.task_up(),
                        KeyCode::Char(' ') => {
                            let dest_index = app.task_lists.len() - 1;
                            app.move_task_to_list(dest_index);
                        },
                        KeyCode::Char('c') => app.cycle_list_color(1),
                        KeyCode::Char('C') => app.cycle_list_color(-1),
                        KeyCode::Char('a') => app.change_state(*prev),
                        KeyCode::Char('b') => app.change_state(AppState::BacklogPopup(prev)),
                        KeyCode::Enter => {
                            if !app.focused_list_is_empty() {
                                app.change_state(AppState::TaskView(Box::new(AppState::ArchivePopup(prev))));
                            }
                        },
                        _ => {}
                    }
                },
                AppState::EditTask(prev) => {
                    match key.code {
                        KeyCode::Char(c) => app.add_to_detail_input(c),
                        KeyCode::Backspace => app.delete_from_detail_input(),
                        KeyCode::Delete => app.clear_focused_input(),
                        KeyCode::Tab => app.next_detail_input(),
                        KeyCode::Enter => {
                            app.save_details_to_task();
                            app.change_state(*prev);
                        }
                        KeyCode::Esc => app.change_state(*prev),
                        _ => {}
                    }
                },
                AppState::CreateTask(prev) => {
                    match key.code {
                        KeyCode::Char(c) => app.add_to_detail_input(c),
                        KeyCode::Backspace => app.delete_from_detail_input(),
                        KeyCode::Delete => app.clear_focused_input(),
                        KeyCode::Tab => app.next_detail_input(),
                        KeyCode::Enter => {
                            app.save_details_to_task();
                            app.change_state(*prev);
                        }
                        KeyCode::Esc => app.change_state(*prev),
                        _ => {}
                    }
                },
                AppState::DeleteTask(prev) => {
                    match key.code {
                        KeyCode::Char('y') => {
                            app.delete_highlighted_task();
                            app.change_state(*prev);
                        },
                        KeyCode::Char('n') => app.change_state(*prev),
                        KeyCode::Enter => {
                            app.delete_highlighted_task();
                            app.change_state(*prev);
                        },
                        KeyCode::Esc => app.change_state(*prev),
                        _ => {}
                    }
                },
                AppState::EditList(prev) => {
                    match key.code {
                        KeyCode::Char(c) => app.add_to_detail_input(c),
                        KeyCode::Backspace => app.delete_from_detail_input(),
                        KeyCode::Delete => app.clear_focused_input(),
                        KeyCode::Enter => {
                            app.save_details_to_list();
                            app.change_state(*prev);
                        }
                        KeyCode::Esc => app.change_state(*prev),
                        _ => {}
                    }
                },
                AppState::CreateList(prev) => {
                    match key.code {
                        KeyCode::Char(c) => app.add_to_detail_input(c),
                        KeyCode::Backspace => app.delete_from_detail_input(),
                        KeyCode::Delete => app.clear_focused_input(),
                        KeyCode::Enter => {
                            app.save_details_to_list();
                            app.change_state(*prev);
                        }
                        KeyCode::Esc => app.change_state(*prev),
                        _ => {}
                    }
                },
                AppState::DeleteList(prev) => {
                    match key.code {
                        KeyCode::Char('y') => {
                            app.delete_focused_list();
                            app.change_state(*prev);
                        },
                        KeyCode::Char('n') => app.change_state(*prev),
                        KeyCode::Enter => {
                            app.delete_focused_list();
                            app.change_state(*prev);
                        },
                        KeyCode::Esc => app.change_state(*prev),
                        _ => {}
                    }
                },
            }
        }
    }

    app.save_changes()?;

    Ok(())
}
