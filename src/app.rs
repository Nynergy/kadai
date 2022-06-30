use std::{cmp, env, fs};

use crate::inputs::*;
use crate::lists::*;

const TRACKER_FILE: &str = "tracker.json";
const BACKLOG_FILE: &str = "backlog.json";
const ARCHIVE_FILE: &str = "archive.json";

#[derive(Clone)]
pub enum AppState {
    ProjectMenu,
    Tracker,
    TaskView(Box<AppState>),
    BacklogPopup(Box<AppState>),
    ArchivePopup(Box<AppState>),
    EditTask(Box<AppState>),
    CreateTask(Box<AppState>),
    DeleteTask(Box<AppState>),
    EditList(Box<AppState>),
    CreateList(Box<AppState>),
    DeleteList(Box<AppState>),
}

pub struct App {
    pub project_title: String,
    pub project_list: ProjectList,
    pub unsaved_changes: bool,
    pub quit: bool,
    pub state: AppState,
    pub task_lists: Vec<TaskList>,
    pub active_list: usize,
    pub backlog: TaskList,
    pub archive: TaskList,
    pub detail_scroll: u16,
    pub task_detail_inputs: Vec<Input>,
    pub active_detail_input: usize,
    pub list_detail_input: Input,
}

impl App {
    pub fn create(project_title: String) -> Result<Self, std::io::Error> {
        let mut app = Self {
            project_title,
            project_list: ProjectList::create()?,
            unsaved_changes: false,
            quit: false,
            state: AppState::Tracker,
            task_lists: Vec::new(),
            active_list: 0,
            backlog: TaskList::default(),
            archive: TaskList::default(),
            detail_scroll: 0,
            task_detail_inputs: vec![Input::new(); 3],
            active_detail_input: 0,
            list_detail_input: Input::new(),
        };

        if app.project_title == "" {
            app.state = AppState::ProjectMenu;
        } else {
            let mut path = env::current_dir()?;
            path.push(app.project_title.clone());
            env::set_current_dir(&path)?;

            app.task_lists = read_tracker_file()?;
            app.backlog = read_backlog_file()?;
            app.archive = read_archive_file()?;
        }

        for i in 0..app.task_lists.len() {
            if !app.task_lists[i].tasks.is_empty() {
                app.task_lists[i].state.select(Some(0));
            }
        }
        if !app.backlog.tasks.is_empty() {
            app.backlog.state.select(Some(0));
        }
        if !app.archive.tasks.is_empty() {
            app.archive.state.select(Some(0));
        }

        Ok(app)
    }

    pub fn select_project(&mut self) -> Result<(), std::io::Error> {
        if let Some(project) = self.get_highlighted_project() {
            self.project_title = project.clone();

            let mut path = env::current_dir()?;
            path.push(project);
            env::set_current_dir(&path)?;

            self.task_lists = read_tracker_file()?;
            self.backlog = read_backlog_file()?;
            self.archive = read_archive_file()?;
        }

        Ok(())
    }

    pub fn get_highlighted_project(&self) -> Option<String> {
        match self.project_list.state.selected() {
            Some(i) => Some(self.project_list.projects[i].clone()),
            None => None
        }
    }

    pub fn set_quit(&mut self, quit: bool) {
        self.quit = quit;
    }

    fn get_focused_list(&self, state: &AppState) -> &TaskList {
        match state {
            AppState::ProjectMenu => unreachable!(),
            AppState::Tracker => &self.task_lists[self.active_list],
            AppState::BacklogPopup(_) => &self.backlog,
            AppState::ArchivePopup(_) => &self.archive,
            AppState::TaskView(prev) => self.get_focused_list(&*prev),
            AppState::EditTask(prev) => self.get_focused_list(&*prev),
            AppState::CreateTask(prev) => self.get_focused_list(&*prev),
            AppState::DeleteTask(prev) => self.get_focused_list(&*prev),
            AppState::EditList(prev) => self.get_focused_list(&*prev),
            AppState::CreateList(prev) => self.get_focused_list(&*prev),
            AppState::DeleteList(prev) => self.get_focused_list(&*prev),
        }
    }


    fn get_mut_focused_list(&mut self, state: &AppState) -> &mut TaskList {
        match state {
            AppState::ProjectMenu => unreachable!(),
            AppState::Tracker => &mut self.task_lists[self.active_list],
            AppState::BacklogPopup(_) => &mut self.backlog,
            AppState::ArchivePopup(_) => &mut self.archive,
            AppState::TaskView(prev) => self.get_mut_focused_list(&*prev),
            AppState::EditTask(prev) => self.get_mut_focused_list(&*prev),
            AppState::CreateTask(prev) => self.get_mut_focused_list(&*prev),
            AppState::DeleteTask(prev) => self.get_mut_focused_list(&*prev),
            AppState::EditList(prev) => self.get_mut_focused_list(&*prev),
            AppState::CreateList(prev) => self.get_mut_focused_list(&*prev),
            AppState::DeleteList(prev) => self.get_mut_focused_list(&*prev),
        }
    }

    pub fn list_down(&mut self) {
        match self.state {
            AppState::ProjectMenu => {
                if !self.project_list.projects.is_empty() {
                    let i = match self.project_list.state.selected() {
                        Some(i) => {
                            if i >= self.project_list.projects.len() - 1 {
                                0
                            } else {
                                i + 1
                            }
                        },
                        None => 0,
                    };
                    self.project_list.state.select(Some(i));
                }
            },
            _ => {
                let list = self.get_mut_focused_list(&self.state.clone());

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
        }
    }

    pub fn list_up(&mut self) {
        match self.state {
            AppState::ProjectMenu => {
                if !self.project_list.projects.is_empty() {
                    let i = match self.project_list.state.selected() {
                        Some(i) => {
                            if i == 0 {
                                self.project_list.projects.len() - 1
                            } else {
                                i - 1
                            }
                        },
                        None => cmp::max(self.project_list.projects.len() - 1, 0),
                    };
                    self.project_list.state.select(Some(i));
                }
            },
            _ => {
                let list = self.get_mut_focused_list(&self.state.clone());

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

    pub fn task_up(&mut self) {
        let list = self.get_mut_focused_list(&self.state.clone());

        if let Some(i) = list.state.selected() {
            if let Some(index) = i.checked_sub(1) {
                list.tasks.swap(i, index);
                list.state.select(Some(index));

                self.unsaved_changes = true;
            }
        }
    }

    pub fn task_down(&mut self) {
        let list = self.get_mut_focused_list(&self.state.clone());

        if let Some(i) = list.state.selected() {
            let mut index = i + 1;
            if index >= list.tasks.len() {
                index = list.tasks.len() - 1;
            }

            list.tasks.swap(i, index);
            list.state.select(Some(index));

            self.unsaved_changes = true;
        }
    }

    pub fn jump_to_list_top(&mut self) {
        match self.state {
            AppState::ProjectMenu => {
                if let Some(_) = self.project_list.state.selected() {
                    self.project_list.state.select(Some(0));
                }
            },
            _ => {
                let list = self.get_mut_focused_list(&self.state.clone());

                if let Some(_) = list.state.selected() {
                    list.state.select(Some(0));
                }
            }
        }
    }

    pub fn jump_to_list_bottom(&mut self) {
        match self.state {
            AppState::ProjectMenu => {
                if let Some(_) = self.project_list.state.selected() {
                    self.project_list.state.select(Some(self.project_list.projects.len() - 1));
                }
            },
            _ => {
                let list = self.get_mut_focused_list(&self.state.clone());

                if let Some(_) = list.state.selected() {
                    list.state.select(Some(list.tasks.len() - 1));
                }
            }
        }
    }

    pub fn list_left(&mut self) {
        if let Some(index) = self.active_list.checked_sub(1) {
            self.task_lists.swap(self.active_list, index);
            self.active_list = index;

            self.unsaved_changes = true;
        }
    }

    pub fn list_right(&mut self) {
        let index = self.active_list + 1;
        if index >= self.task_lists.len() {
            return;
        }

        self.task_lists.swap(self.active_list, index);
        self.active_list = index;

        self.unsaved_changes = true;
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

                    self.unsaved_changes = true;
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

                    self.unsaved_changes = true;
                },
                None => return,
            }
        }
    }

    pub fn move_task_to_list(&mut self, index: usize) {
        if index >= self.task_lists.len() {
            return;
        }

        let list = self.get_mut_focused_list(&self.state.clone());

        if let Some(i) = list.state.selected() {
            let task = list.tasks.remove(i);
            if list.tasks.len() == 0 {
                list.state.select(None);
            } else if i == list.tasks.len() {
                list.state.select(Some(i - 1));
            }

            let dest = &mut self.task_lists[index];
            dest.tasks.push(task);
            if dest.tasks.len() == 1 {
                dest.state.select(Some(0));
            }

            self.unsaved_changes = true;
        }
    }

    pub fn move_task_to_backlog(&mut self) {
        let list = &mut self.task_lists[self.active_list];

        if let Some(i) = list.state.selected() {
            let task = list.tasks.remove(i);
            if list.tasks.len() == 0 {
                list.state.select(None);
            } else if i == list.tasks.len() {
                list.state.select(Some(i - 1));
            }

            let dest = &mut self.backlog;
            dest.tasks.push(task);
            if dest.tasks.len() == 1 {
                dest.state.select(Some(0));
            }

            self.unsaved_changes = true;
        }
    }

    pub fn move_task_to_archive(&mut self) {
        let list = &mut self.task_lists[self.active_list];
        if let Some(i) = list.state.selected() {
            let task = list.tasks.remove(i);
            if list.tasks.len() == 0 {
                list.state.select(None);
            } else if i == list.tasks.len() {
                list.state.select(Some(i - 1));
            }

            let dest = &mut self.archive;
            dest.tasks.push(task);
            if dest.tasks.len() == 1 {
                dest.state.select(Some(0));
            }

            self.unsaved_changes = true;
        }
    }

    pub fn change_state(&mut self, state: AppState) {
        self.state = state;
    }

    pub fn get_selected_task(&self) -> Option<&Task> {
        let list = self.get_focused_list(&self.state);

        match list.state.selected() {
            Some(i) => Some(&list.tasks[i]),
            None => None
        }
    }

    pub fn focused_list_is_empty(&self) -> bool {
        let list = self.get_focused_list(&self.state);

        list.tasks.is_empty()
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
        let list = self.get_mut_focused_list(&self.state.clone());

        let mut new_color = list.color_index as i8 + amount;
        if new_color < 1 {
            new_color = 7;
        } else if new_color > 7 {
            new_color = 1;
        }
        list.color_index = new_color as u8;

        self.unsaved_changes = true;
    }

    pub fn save_changes(&mut self) -> Result<(), std::io::Error> {
        save_tracker_file(&self.task_lists)?;
        save_backlog_file(&self.backlog)?;
        save_archive_file(&self.archive)?;
        self.unsaved_changes = false;
        Ok(())
    }

    pub fn populate_task_detail_inputs(&mut self) {
        if let Some(task) = self.get_selected_task() {
            let description: String;
            match &task.description {
                Some(d) => description = d.to_string(),
                None => description = String::new()
            }
            let category: String;
            match &task.category {
                Some(c) => category = c.to_string(),
                None => category = String::new()
            }

            self.task_detail_inputs[0] = Input::from(task.summary.clone());
            self.task_detail_inputs[1] = Input::from(description);
            self.task_detail_inputs[2] = Input::from(category);
        }
    }

    pub fn add_to_detail_input(&mut self, c: char) {
        match self.state {
            AppState::EditTask(_) => {
                self.task_detail_inputs[self.active_detail_input].push(c);
            },
            AppState::CreateTask(_) => {
                self.task_detail_inputs[self.active_detail_input].push(c);
            },
            AppState::EditList(_) => {
                self.list_detail_input.push(c);
            },
            AppState::CreateList(_) => {
                self.list_detail_input.push(c);
            },
            _ => {}
        }
    }

    pub fn delete_from_detail_input(&mut self) {
        match self.state {
            AppState::EditTask(_) => {
                self.task_detail_inputs[self.active_detail_input].pop();
            },
            AppState::CreateTask(_) => {
                self.task_detail_inputs[self.active_detail_input].pop();
            },
            AppState::EditList(_) => {
                self.list_detail_input.pop();
            },
            AppState::CreateList(_) => {
                self.list_detail_input.pop();
            },
            _ => {}
        }
    }

    pub fn next_detail_input(&mut self) {
        self.active_detail_input += 1;
        self.active_detail_input %= self.task_detail_inputs.len();
    }

    pub fn prev_detail_input(&mut self) {
        let res = self.active_detail_input.checked_sub(1);
        match res {
            Some(i) => self.active_detail_input = i,
            None => self.active_detail_input = self.task_detail_inputs.len() - 1
        }
    }

    pub fn reset_active_detail_input(&mut self) {
        self.active_detail_input = 0;
    }

    pub fn save_details_to_task(&mut self) {
        let summary = self.task_detail_inputs[0].extract();
        let desc = self.task_detail_inputs[1].extract();
        let cat = self.task_detail_inputs[2].extract();

        let description: Option<String>;
        if desc == "" {
            description = None;
        } else {
            description = Some(desc);
        }

        let category: Option<String>;
        if cat == "" {
            category = None;
        } else {
            category = Some(cat);
        }

        let new_task = Task {
            summary,
            description,
            category,
        };

        match &self.state {
            AppState::EditTask(prev) => {
                let list = match **prev {
                    AppState::Tracker => &mut self.task_lists[self.active_list],
                    AppState::BacklogPopup(_) => &mut self.backlog,
                    AppState::ArchivePopup(_) => &mut self.archive,
                    _ => return
                };

                if let Some(i) = list.state.selected() {
                    list.tasks.remove(i);
                    list.tasks.insert(i, new_task);
                }
            },
            AppState::CreateTask(prev) => {
                let list = match **prev {
                    AppState::Tracker => &mut self.task_lists[self.active_list],
                    AppState::BacklogPopup(_) => &mut self.backlog,
                    AppState::ArchivePopup(_) => &mut self.archive,
                    _ => return
                };

                list.tasks.push(new_task);
                if list.tasks.len() == 1 {
                    list.state.select(Some(0));
                }
            },
            _ => {}
        }

        self.unsaved_changes = true;
    }

    pub fn clear_detail_inputs(&mut self) {
        for i in 0..self.task_detail_inputs.len() {
            self.task_detail_inputs[i].clear();
        }
    }

    pub fn clear_focused_input(&mut self) {
        match self.state {
            AppState::EditTask(_) => {
                self.task_detail_inputs[self.active_detail_input].clear();
            },
            AppState::CreateTask(_) => {
                self.task_detail_inputs[self.active_detail_input].clear();
            },
            AppState::EditList(_) => {
                self.list_detail_input.clear();
            },
            AppState::CreateList(_) => {
                self.list_detail_input.clear();
            },
            _ => {}
        }
    }

    pub fn populate_list_detail_inputs(&mut self) {
        let list = self.get_focused_list(&self.state);

        self.list_detail_input = Input::from(list.name.clone());
    }

    pub fn save_details_to_list(&mut self) {
        let name = self.list_detail_input.extract();

        match &self.state {
            AppState::EditList(prev) => {
                let list = match **prev {
                    AppState::Tracker => &self.task_lists[self.active_list],
                    AppState::BacklogPopup(_) => &self.backlog,
                    AppState::ArchivePopup(_) => &self.archive,
                    _ => return
                };

                let new_list = TaskList {
                    name,
                    color_index: list.color_index,
                    state: list.state.clone(),
                    tasks: list.tasks.clone(),
                };

                match **prev {
                    AppState::Tracker => self.task_lists[self.active_list] = new_list,
                    AppState::BacklogPopup(_) => self.backlog = new_list,
                    AppState::ArchivePopup(_) => self.archive = new_list,
                    _ => return
                }
            },
            AppState::CreateList(_prev) => {
                self.task_lists.push(TaskList::from(name));
            },
            _ => {}
        }

        self.unsaved_changes = true;
    }

    pub fn delete_highlighted_task(&mut self) {
        if let AppState::DeleteTask(prev) = &self.state {
            let list = match **prev {
                AppState::Tracker => &mut self.task_lists[self.active_list],
                AppState::BacklogPopup(_) => &mut self.backlog,
                AppState::ArchivePopup(_) => &mut self.archive,
                _ => return
            };

            if let Some(i) = list.state.selected() {
                list.tasks.remove(i);
                if list.tasks.is_empty() {
                    list.state.select(None);
                } else if i >= list.tasks.len() {
                    list.state.select(Some(i - 1));
                }

                self.unsaved_changes = true;
            }
        }
    }

    pub fn delete_focused_list(&mut self) {
        self.task_lists.remove(self.active_list);

        if self.task_lists.is_empty() {
            self.create_default_list();
        } else if self.active_list >= self.task_lists.len() {
            self.active_list -= 1;
        }

        self.unsaved_changes = true;
    }

    fn create_default_list(&mut self) {
        self.task_lists.push(TaskList::default());
    }

    pub fn clear_list_inputs(&mut self) {
        self.list_detail_input.clear();
    }

    pub fn input_left(&mut self) {
        match self.state {
            AppState::EditTask(_) => {
                self.task_detail_inputs[self.active_detail_input].move_left();
            },
            AppState::CreateTask(_) => {
                self.task_detail_inputs[self.active_detail_input].move_left();
            },
            AppState::EditList(_) => {
                self.list_detail_input.move_left();
            },
            AppState::CreateList(_) => {
                self.list_detail_input.move_left();
            },
            _ => {}
        }
    }

    pub fn input_right(&mut self) {
        match self.state {
            AppState::EditTask(_) => {
                self.task_detail_inputs[self.active_detail_input].move_right();
            },
            AppState::CreateTask(_) => {
                self.task_detail_inputs[self.active_detail_input].move_right();
            },
            AppState::EditList(_) => {
                self.list_detail_input.move_right();
            },
            AppState::CreateList(_) => {
                self.list_detail_input.move_right();
            },
            _ => {}
        }
    }

    pub fn input_start(&mut self) {
        match self.state {
            AppState::EditTask(_) => {
                self.task_detail_inputs[self.active_detail_input].move_start();
            },
            AppState::CreateTask(_) => {
                self.task_detail_inputs[self.active_detail_input].move_start();
            },
            AppState::EditList(_) => {
                self.list_detail_input.move_start();
            },
            AppState::CreateList(_) => {
                self.list_detail_input.move_start();
            },
            _ => {}
        }
    }

    pub fn input_end(&mut self) {
        match self.state {
            AppState::EditTask(_) => {
                self.task_detail_inputs[self.active_detail_input].move_end();
            },
            AppState::CreateTask(_) => {
                self.task_detail_inputs[self.active_detail_input].move_end();
            },
            AppState::EditList(_) => {
                self.list_detail_input.move_end();
            },
            AppState::CreateList(_) => {
                self.list_detail_input.move_end();
            },
            _ => {}
        }
    }

    pub fn input_jump_to_space_left(&mut self) {
        match self.state {
            AppState::EditTask(_) => {
                self.task_detail_inputs[self.active_detail_input].move_to_prev_space();
            },
            AppState::CreateTask(_) => {
                self.task_detail_inputs[self.active_detail_input].move_to_prev_space();
            },
            AppState::EditList(_) => {
                self.list_detail_input.move_to_prev_space();
            },
            AppState::CreateList(_) => {
                self.list_detail_input.move_to_prev_space();
            },
            _ => {}
        }
    }

    pub fn input_jump_to_space_right(&mut self) {
        match self.state {
            AppState::EditTask(_) => {
                self.task_detail_inputs[self.active_detail_input].move_to_next_space();
            },
            AppState::CreateTask(_) => {
                self.task_detail_inputs[self.active_detail_input].move_to_next_space();
            },
            AppState::EditList(_) => {
                self.list_detail_input.move_to_next_space();
            },
            AppState::CreateList(_) => {
                self.list_detail_input.move_to_next_space();
            },
            _ => {}
        }
    }
}

fn read_tracker_file() -> Result<Vec<TaskList>, std::io::Error> {
    let mut path = env::current_dir()?;
    path.push(TRACKER_FILE);

    if !path.exists() {
        save_tracker_file(&vec![TaskList::default()])?;
    }

    let file_contents = fs::read_to_string(path.as_path())?;
    let parsed: Vec<TaskList> = serde_json::from_str(&file_contents)?;
    Ok(parsed)
}

fn read_backlog_file() -> Result<TaskList, std::io::Error> {
    let mut path = env::current_dir()?;
    path.push(BACKLOG_FILE);

    if !path.exists() {
        save_backlog_file(&TaskList::empty_backlog())?;
    }

    let file_contents = fs::read_to_string(path.as_path())?;
    let parsed: TaskList = serde_json::from_str(&file_contents)?;
    Ok(parsed)
}

fn read_archive_file() -> Result<TaskList, std::io::Error> {
    let mut path = env::current_dir()?;
    path.push(ARCHIVE_FILE);

    if !path.exists() {
        save_archive_file(&TaskList::empty_archive())?;
    }

    let file_contents = fs::read_to_string(path.as_path())?;
    let parsed: TaskList = serde_json::from_str(&file_contents)?;
    Ok(parsed)
}

fn save_tracker_file(data: &Vec<TaskList>) -> Result<(), std::io::Error> {
    let mut path = env::current_dir()?;
    path.push(TRACKER_FILE);
    let json_data = serde_json::to_string_pretty(data)?;
    fs::write(path.as_path(), json_data)?;
    Ok(())
}

fn save_backlog_file(data: &TaskList) -> Result<(), std::io::Error> {
    let mut path = env::current_dir()?;
    path.push(BACKLOG_FILE);
    let json_data = serde_json::to_string_pretty(data)?;
    fs::write(path.as_path(), json_data)?;
    Ok(())
}

fn save_archive_file(data: &TaskList) -> Result<(), std::io::Error> {
    let mut path = env::current_dir()?;
    path.push(ARCHIVE_FILE);
    let json_data = serde_json::to_string_pretty(data)?;
    fs::write(path.as_path(), json_data)?;
    Ok(())
}
