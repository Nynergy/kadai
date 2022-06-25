use serde::{Deserialize, Serialize};
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
}
