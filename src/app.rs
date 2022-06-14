use std::cmp;
use tui::widgets::ListState;

const NUM_LISTS: usize = 3;

pub struct App {
    pub list_states: Vec<ListState>,
    pub list_items: Vec<Vec<String>>,
    pub active_list: usize,
}

impl App {
    pub fn new() -> Self {
        let mut app = Self {
            list_states: vec![ListState::default(); NUM_LISTS],
            list_items: vec![
                vec![
                    "Task 0".to_string(),
                    "Task 1".to_string(),
                    "Task 2".to_string(),
                    "Task 3".to_string(),
                    "Task 4".to_string(),
                    "Task 5".to_string(),
                    "Task 6".to_string(),
                    "Task 7".to_string(),
                    "Task 8".to_string(),
                    "Task 9".to_string(),
                ],
                vec![
                    "Task 10".to_string(),
                    "Task 11".to_string(),
                    "Task 12".to_string(),
                ],
                vec![
                    "Task 13".to_string(),
                    "Task 14".to_string(),
                ]
            ],
            active_list: 0,
        };

        for i in 0..NUM_LISTS {
            app.list_states[i].select(Some(0));
        }
        app
    }

    pub fn list_down(&mut self) {
        let list_state = &mut self.list_states[self.active_list];
        let list_items = &self.list_items[self.active_list];

        if !list_items.is_empty() {
            let i = match list_state.selected() {
                Some(i) => {
                    if i >= list_items.len() - 1 {
                        0
                    } else {
                        i + 1
                    }
                },
                None => 0,
            };
            list_state.select(Some(i));
        }
    }

    pub fn list_up(&mut self) {
        let list_state = &mut self.list_states[self.active_list];
        let list_items = &self.list_items[self.active_list];

        if !list_items.is_empty() {
            let i = match list_state.selected() {
                Some(i) => {
                    if i == 0 {
                        list_items.len() - 1
                    } else {
                        i - 1
                    }
                },
                None => cmp::max(list_items.len() - 1, 0),
            };
            list_state.select(Some(i));
        }
    }

    pub fn next_list(&mut self) {
        self.active_list = (self.active_list + 1) % NUM_LISTS;
    }

    pub fn prev_list(&mut self) {
        if self.active_list == 0 {
            self.active_list = NUM_LISTS - 1;
        } else {
            self.active_list -= 1;
        }
    }

    pub fn move_task_to_next_list(&mut self) {
        if self.active_list != NUM_LISTS - 1 {
            let list_state = &mut self.list_states[self.active_list];
            let list_items = &mut self.list_items[self.active_list];

            match list_state.selected() {
                Some(i) => {
                    let task = list_items.remove(i);
                    if list_items.len() == 0 {
                        list_state.select(None);
                    } else if i == list_items.len() {
                        list_state.select(Some(i - 1));
                    }

                    self.next_list();
                    let list_state = &mut self.list_states[self.active_list];
                    let list_items = &mut self.list_items[self.active_list];

                    list_items.push(task);
                    list_state.select(Some(list_items.len() - 1));
                },
                None => return,
            }
        }
    }

    pub fn move_task_to_prev_list(&mut self) {
        if self.active_list != 0 {
            let list_state = &mut self.list_states[self.active_list];
            let list_items = &mut self.list_items[self.active_list];

            match list_state.selected() {
                Some(i) => {
                    let task = list_items.remove(i);
                    if list_items.len() == 0 {
                        list_state.select(None);
                    } else if i == list_items.len() {
                        list_state.select(Some(i - 1));
                    }

                    self.prev_list();
                    let list_state = &mut self.list_states[self.active_list];
                    let list_items = &mut self.list_items[self.active_list];

                    list_items.push(task);
                    list_state.select(Some(list_items.len() - 1));
                },
                None => return,
            }
        }
    }
}
