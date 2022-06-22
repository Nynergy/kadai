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
    pub name: String,
    pub color_index: u8,
    pub state: ListState,
    pub tasks: Vec<Task>,
}

impl TaskList {
    // TODO: No hardcoded tasks, set them using TaskList::tasks()
    pub fn new() -> Self {
        Self {
            name: "".to_string(),
            color_index: 7,
            state: ListState::default(),
            tasks: vec![
                Task::from(
                    "Task 1".to_string(),
                )
                .description("This is a description that is so long, it simply must wrap, otherwise the app will crash and burn. In fact, even the task details popup can't properly render it without wrapping the text first.".to_string()),
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

    pub fn name(mut self, name: String) -> Self {
        self.name = name;
        self
    }

    pub fn color_index(mut self, color_index: u8) -> Self {
        self.color_index = color_index;
        self
    }
}
