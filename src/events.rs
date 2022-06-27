use crossterm::{
    event::{
        self,
        Event,
        KeyCode,
        KeyEvent
    }
};
use std::io;

use crate::app::*;

pub fn handle_events(app: &mut App) -> io::Result<()> {
    if let Event::Key(key) = event::read()? {
        let state = app.state.clone();
        match state {
            AppState::Tracker => {
                handle_tracker_events(key, app, state);
            },
            AppState::TaskView(prev) => {
                handle_task_view_events(key, app, *prev);
            },
            AppState::BacklogPopup(prev) => {
                handle_backlog_popup_events(key, app, *prev);
            },
            AppState::ArchivePopup(prev) => {
                handle_archive_popup_events(key, app, *prev);
            },
            AppState::EditTask(prev) => {
                handle_edit_task_events(key, app, *prev);
            },
            AppState::CreateTask(prev) => {
                handle_create_task_events(key, app, *prev);
            },
            AppState::DeleteTask(prev) => {
                handle_delete_task_events(key, app, *prev);
            },
            AppState::EditList(prev) => {
                handle_edit_list_events(key, app, *prev);
            },
            AppState::CreateList(prev) => {
                handle_create_list_events(key, app, *prev);
            },
            AppState::DeleteList(prev) => {
                handle_delete_list_events(key, app, *prev);
            },
        }
    }

    Ok(())
}

fn handle_tracker_events(key: KeyEvent, app: &mut App, state: AppState) {
    match key.code {
        KeyCode::Char('q') => app.set_quit(true),
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
        KeyCode::Char('g') => app.jump_to_list_top(),
        KeyCode::Char('G') => app.jump_to_list_bottom(),
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
}

fn handle_task_view_events(key: KeyEvent, app: &mut App, prev: AppState) {
    match key.code {
        KeyCode::Char('q') => app.set_quit(true),
        KeyCode::Char('j') => app.scroll_details(1),
        KeyCode::Char('k') => app.scroll_details(-1),
        KeyCode::Enter => {
            app.reset_scroll();
            app.change_state(prev);
        },
        _ => {}
    }
}

fn handle_backlog_popup_events(key: KeyEvent, app: &mut App, prev: AppState) {
    match key.code {
        KeyCode::Char('q') => app.set_quit(true),
        KeyCode::Char('n') => {
            app.clear_detail_inputs();
            app.reset_active_detail_input();
            app.change_state(
                AppState::CreateTask(
                    Box::new(
                        AppState::BacklogPopup(
                            Box::new(prev)
                        )
                    )
                )
            );
        },
        KeyCode::Char('d') => {
            if !app.focused_list_is_empty() {
                app.change_state(
                    AppState::DeleteTask(
                        Box::new(
                            AppState::BacklogPopup(
                                Box::new(prev)
                            )
                        )
                    )
                );
            }
        },
        KeyCode::Char('e') => {
            if !app.focused_list_is_empty() {
                app.populate_task_detail_inputs();
                app.reset_active_detail_input();
                app.change_state(
                    AppState::EditTask(
                        Box::new(
                            AppState::BacklogPopup(
                                Box::new(prev)
                            )
                        )
                    )
                );
            }
        },
        KeyCode::Char('j') => app.list_down(),
        KeyCode::Char('k') => app.list_up(),
        KeyCode::Char('J') => app.task_down(),
        KeyCode::Char('K') => app.task_up(),
        KeyCode::Char('g') => app.jump_to_list_top(),
        KeyCode::Char('G') => app.jump_to_list_bottom(),
        KeyCode::Char(' ') => app.move_task_to_list(0),
        KeyCode::Char('c') => app.cycle_list_color(1),
        KeyCode::Char('C') => app.cycle_list_color(-1),
        KeyCode::Char('b') => app.change_state(prev),
        KeyCode::Char('a') => app.change_state(
            AppState::ArchivePopup(
                Box::new(prev)
            )
        ),
        KeyCode::Enter => {
            if !app.focused_list_is_empty() {
                app.change_state(
                    AppState::TaskView(
                        Box::new(
                            AppState::BacklogPopup(
                                Box::new(prev)
                            )
                        )
                    )
                );
            }
        },
        _ => {}
    }
}

fn handle_archive_popup_events(key: KeyEvent, app: &mut App, prev: AppState) {
    match key.code {
        KeyCode::Char('q') => app.set_quit(true),
        KeyCode::Char('d') => {
            if !app.focused_list_is_empty() {
                app.change_state(
                    AppState::DeleteTask(
                        Box::new(
                            AppState::ArchivePopup(
                                Box::new(prev)
                            )
                        )
                    )
                );
            }
        },
        KeyCode::Char('j') => app.list_down(),
        KeyCode::Char('k') => app.list_up(),
        KeyCode::Char('J') => app.task_down(),
        KeyCode::Char('K') => app.task_up(),
        KeyCode::Char('g') => app.jump_to_list_top(),
        KeyCode::Char('G') => app.jump_to_list_bottom(),
        KeyCode::Char(' ') => {
            let dest_index = app.task_lists.len() - 1;
            app.move_task_to_list(dest_index);
        },
        KeyCode::Char('c') => app.cycle_list_color(1),
        KeyCode::Char('C') => app.cycle_list_color(-1),
        KeyCode::Char('a') => app.change_state(prev),
        KeyCode::Char('b') => app.change_state(
            AppState::BacklogPopup(
                Box::new(prev)
            )
        ),
        KeyCode::Enter => {
            if !app.focused_list_is_empty() {
                app.change_state(
                    AppState::TaskView(
                        Box::new(
                            AppState::ArchivePopup(
                                Box::new(prev)
                            )
                        )
                    )
                );
            }
        },
        _ => {}
    }
}

fn handle_edit_task_events(key: KeyEvent, app: &mut App, prev: AppState) {
    match key.code {
        KeyCode::Char(c) => app.add_to_detail_input(c),
        KeyCode::Backspace => app.delete_from_detail_input(),
        KeyCode::Delete => app.clear_focused_input(),
        KeyCode::Tab => app.next_detail_input(),
        KeyCode::Enter => {
            app.save_details_to_task();
            app.change_state(prev);
        }
        KeyCode::Esc => app.change_state(prev),
        _ => {}
    }
}

fn handle_create_task_events(key: KeyEvent, app: &mut App, prev: AppState) {
    match key.code {
        KeyCode::Char(c) => app.add_to_detail_input(c),
        KeyCode::Backspace => app.delete_from_detail_input(),
        KeyCode::Delete => app.clear_focused_input(),
        KeyCode::Tab => app.next_detail_input(),
        KeyCode::Enter => {
            app.save_details_to_task();
            app.change_state(prev);
        }
        KeyCode::Esc => app.change_state(prev),
        _ => {}
    }
}

fn handle_delete_task_events(key: KeyEvent, app: &mut App, prev: AppState) {
    match key.code {
        KeyCode::Char('y') => {
            app.delete_highlighted_task();
            app.change_state(prev);
        },
        KeyCode::Char('n') => app.change_state(prev),
        KeyCode::Enter => {
            app.delete_highlighted_task();
            app.change_state(prev);
        },
        KeyCode::Esc => app.change_state(prev),
        _ => {}
    }
}

fn handle_edit_list_events(key: KeyEvent, app: &mut App, prev: AppState) {
    match key.code {
        KeyCode::Char(c) => app.add_to_detail_input(c),
        KeyCode::Backspace => app.delete_from_detail_input(),
        KeyCode::Delete => app.clear_focused_input(),
        KeyCode::Enter => {
            app.save_details_to_list();
            app.change_state(prev);
        }
        KeyCode::Esc => app.change_state(prev),
        _ => {}
    }
}

fn handle_create_list_events(key: KeyEvent, app: &mut App, prev: AppState) {
    match key.code {
        KeyCode::Char(c) => app.add_to_detail_input(c),
        KeyCode::Backspace => app.delete_from_detail_input(),
        KeyCode::Delete => app.clear_focused_input(),
        KeyCode::Enter => {
            app.save_details_to_list();
            app.change_state(prev);
        }
        KeyCode::Esc => app.change_state(prev),
        _ => {}
    }
}

fn handle_delete_list_events(key: KeyEvent, app: &mut App, prev: AppState) {
    match key.code {
        KeyCode::Char('y') => {
            app.delete_focused_list();
            app.change_state(prev);
        },
        KeyCode::Char('n') => app.change_state(prev),
        KeyCode::Enter => {
            app.delete_focused_list();
            app.change_state(prev);
        },
        KeyCode::Esc => app.change_state(prev),
        _ => {}
    }
}
