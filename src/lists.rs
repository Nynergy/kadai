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
