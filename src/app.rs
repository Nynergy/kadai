use std::cmp;

use crate::task_list::*;

pub struct App {
    pub task_lists: Vec<TaskList>,
    pub active_list: usize,
}

impl App {
    pub fn new() -> Self {
        let mut app = Self {
            task_lists: vec![
                TaskList::new(),
                TaskList::new(),
                TaskList::new(),
            ],
            active_list: 0,
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
}
