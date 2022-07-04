use serde::{Deserialize, Serialize};
use std::fs;
use tui::widgets::ListState;

#[derive(Clone, Deserialize, Serialize)]
pub struct Task {
    pub summary: String,
    pub description: Option<String>,
    pub category: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct TaskList {
    pub name: String,
    pub color_index: u8,
    #[serde(skip)]
    pub state: ListState,
    pub tasks: Vec<Task>,
}

impl TaskList {
    pub fn default() -> Self {
        Self {
            name: "Tasks".to_string(),
            color_index: 7,
            state: ListState::default(),
            tasks: Vec::new(),
        }
    }

    pub fn from(name: String) -> Self {
        Self {
            name,
            color_index: 7,
            state: ListState::default(),
            tasks: Vec::new(),
        }
    }

    pub fn empty_backlog() -> Self {
        Self {
            name: "Backlog".to_string(),
            color_index: 6,
            state: ListState::default(),
            tasks: Vec::new(),
        }
    }

    pub fn empty_archive() -> Self {
        Self {
            name: "Archive".to_string(),
            color_index: 1,
            state: ListState::default(),
            tasks: Vec::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.tasks.is_empty()
    }

    pub fn select(&mut self, index: Option<usize>) {
        self.state.select(index);
    }

    pub fn get_selected_index(&self) -> Option<usize> {
        self.state.selected()
    }

    pub fn len(&self) -> usize {
        self.tasks.len()
    }

    pub fn swap(&mut self, i1: usize, i2: usize) {
        self.tasks.swap(i1, i2);
    }

    pub fn remove(&mut self, i: usize) -> Task {
        self.tasks.remove(i)
    }

    pub fn push(&mut self, task: Task) {
        self.tasks.push(task);
    }

    pub fn insert(&mut self, i: usize, task: Task) {
        self.tasks.insert(i, task);
    }
}

pub struct ProjectList {
    pub state: ListState,
    pub projects: Vec<String>,
}

impl ProjectList {
    pub fn create() -> Result<Self, std::io::Error> {
        let mut list = Self {
            state: ListState::default(),
            projects: get_projects()?,
        };

        if !list.projects.is_empty() {
            list.state.select(Some(0));
        }

        Ok(list)
    }

    pub fn is_empty(&self) -> bool {
        self.projects.is_empty()
    }

    pub fn select(&mut self, index: Option<usize>) {
        self.state.select(index);
    }

    pub fn get_selected_index(&self) -> Option<usize> {
        self.state.selected()
    }

    pub fn len(&self) -> usize {
        self.projects.len()
    }

    pub fn remove(&mut self, i: usize) -> String {
        self.projects.remove(i)
    }

    pub fn push(&mut self, project: String) {
        self.projects.push(project);
    }

    pub fn insert(&mut self, i: usize, project: String) {
        self.projects.insert(i, project);
    }
}

fn get_projects() -> Result<Vec<String>, std::io::Error> {
    let mut dirs = Vec::new();
    let paths = fs::read_dir("./")?;
    for path in paths {
        if let Ok(path) = path {
            if path.path().is_dir() {
                if let Some(filename) = path.path().to_str() {
                    let filename = &filename.to_string()[2..];
                    dirs.push(filename.to_string());
                }
            }
        }
    }

    Ok(dirs)
}
