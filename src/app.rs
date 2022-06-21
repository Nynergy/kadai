use std::cmp;

use crate::task_list::*;

pub enum AppState {
    Tracker,
    TaskView,
}

pub struct App {
    pub state: AppState,
    pub task_lists: Vec<TaskList>,
    pub active_list: usize,
    pub detail_scroll: u16,
}

impl App {
    pub fn new() -> Self {
        let mut app = Self {
            state: AppState::Tracker,
            task_lists: vec![
                TaskList::new()
                    .name("Planned".to_string())
                    .color_index(5),
                TaskList::new()
                    .name("In Progress".to_string())
                    .color_index(3),
                TaskList::new()
                    .name("Completed".to_string())
                    .color_index(2),
            ],
            active_list: 0,
            detail_scroll: 0,
        };

        for i in 0..app.task_lists.len() {
            app.task_lists[i].state.select(Some(0));
        }

        app
    }

    pub fn list_down(&mut self) {
        let list = &mut self.task_lists[self.active_list];

        if !list.tasks.is_empty() {
            let i = match list.state.selected() {
                Some(i) => {
                    if i >= list.tasks.len() - 1 {
                        0
                    } else {
                        i + 1
                    }
                },
                None => 0,
            };
            list.state.select(Some(i));
        }
    }

    pub fn list_up(&mut self) {
        let list = &mut self.task_lists[self.active_list];

        if !list.tasks.is_empty() {
            let i = match list.state.selected() {
                Some(i) => {
                    if i == 0 {
                        list.tasks.len() - 1
                    } else {
                        i - 1
                    }
                },
                None => cmp::max(list.tasks.len() - 1, 0),
            };
            list.state.select(Some(i));
        }
    }

    pub fn next_list(&mut self) {
        self.active_list = (self.active_list + 1) % self.task_lists.len();
    }

    pub fn prev_list(&mut self) {
        if self.active_list == 0 {
            self.active_list = self.task_lists.len() - 1;
        } else {
            self.active_list -= 1;
        }
    }

    pub fn move_task_to_next_list(&mut self) {
        if self.active_list != self.task_lists.len() - 1 {
            let list = &mut self.task_lists[self.active_list];

            match list.state.selected() {
                Some(i) => {
                    let task = list.tasks.remove(i);
                    if list.tasks.len() == 0 {
                        list.state.select(None);
                    } else if i == list.tasks.len() {
                        list.state.select(Some(i - 1));
                    }

                    self.next_list();
                    let list = &mut self.task_lists[self.active_list];

                    list.tasks.push(task);
                    list.state.select(Some(list.tasks.len() - 1));
                },
                None => return,
            }
        }
    }

    pub fn move_task_to_prev_list(&mut self) {
        if self.active_list != 0 {
            let list = &mut self.task_lists[self.active_list];

            match list.state.selected() {
                Some(i) => {
                    let task = list.tasks.remove(i);
                    if list.tasks.len() == 0 {
                        list.state.select(None);
                    } else if i == list.tasks.len() {
                        list.state.select(Some(i - 1));
                    }

                    self.prev_list();
                    let list = &mut self.task_lists[self.active_list];

                    list.tasks.push(task);
                    list.state.select(Some(list.tasks.len() - 1));
                },
                None => return,
            }
        }
    }

    pub fn change_state(&mut self, state: AppState) {
        self.state = state;
    }

    pub fn get_selected_task(&self) -> Option<&Task> {
        let list = &self.task_lists[self.active_list];
        match list.state.selected() {
            Some(i) => Some(&list.tasks[i]),
            None => None
        }
    }

    pub fn focused_list_is_empty(&self) -> bool {
        self.task_lists[self.active_list].tasks.is_empty()
    }

    pub fn scroll_details(&mut self, amount: i16) {
        let mut new_scroll = self.detail_scroll as i16 + amount;
        if new_scroll < 0 {
            new_scroll = 0;
        }
        self.detail_scroll = new_scroll as u16;
    }

    pub fn reset_scroll(&mut self) {
        self.detail_scroll = 0;
    }

    pub fn cycle_list_color(&mut self, amount: i8) {
        let list = &mut self.task_lists[self.active_list];
        let mut new_color = list.color_index as i8 + amount;
        if new_color < 1 {
            new_color = 7;
        } else if new_color > 7 {
            new_color = 1;
        }
        list.color_index = new_color as u8;
    }
}
