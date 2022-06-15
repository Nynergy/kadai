use tui::widgets::ListState;

pub struct Task {
    pub summary: String,
    pub description: Option<String>,
    pub category: Option<String>,
}

impl Task {
    pub fn from(summary: String) -> Self {
        Self {
            summary,
            description: None,
            category: None,
        }
    }

    pub fn description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    pub fn category(mut self, category: String) -> Self {
        self.category = Some(category);
        self
    }
}

pub struct TaskList {
    pub state: ListState,
    pub tasks: Vec<Task>,
}

impl TaskList {
    pub fn new() -> Self {
        Self {
            state: ListState::default(),
            tasks: vec![
                Task::from(
                    "Task 1".to_string(),
                )
                .description("This is a description that is so long, it simply must wrap, otherwise the app will crash and burn.".to_string()),
                Task::from(
                    "Task 2 with a name so long it will be truncated in order to fit into the task box".to_string(),
                ),
                Task::from(
                    "Task 3".to_string(),
                )
                .category("TODO".to_string()),
                Task::from(
                    "Task 4".to_string(),
                )
                .category("Personal long category".to_string()),
            ],
        }
    }
}
