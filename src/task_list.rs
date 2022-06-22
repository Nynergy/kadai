use serde::{Deserialize, Serialize};
use tui::widgets::ListState;

#[derive(Deserialize, Serialize)]
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
