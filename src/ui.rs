use std::{
    borrow::Cow,
    cmp
};
use textwrap::{Options, WrapAlgorithm, wrap};
use tui::{
    backend::Backend,
    buffer::Buffer,
    layout::{
        Alignment,
        Constraint,
        Direction,
        Layout,
        Margin,
        Rect
    },
    style::{Color, Modifier, Style},
    symbols::line,
    text::{Span, Spans},
    widgets::{
        Block,
        Borders,
        BorderType,
        Clear,
        List,
        ListItem,
        Paragraph,
        Widget,
        Wrap
    },
    Frame,
};

use crate::app::*;
use crate::inputs::*;
use crate::lists::*;

macro_rules! raw_para {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_para = Vec::new();
            $(
                temp_para.push(
                    Spans::from(
                        Span::raw($x)
                    )
                );
            )*
            temp_para
        }
    };
}

struct CustomBorder {
    title: String,
    title_style: Style,
    border_style: Style,
}

impl CustomBorder {
    fn new() -> Self {
        Self {
            title: "".to_string(),
            title_style: Style::default(),
            border_style: Style::default(),
        }
    }

    fn title(mut self, title: String) -> Self {
        self.title = title;
        self
    }

    fn title_style(mut self, title_style: Style) -> Self {
        self.title_style = title_style;
        self
    }

    fn border_style(mut self, border_style: Style) -> Self {
        self.border_style = border_style;
        self
    }
}

impl Widget for CustomBorder {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Border Lines
        let mut line = String::new();
        line.push_str(line::VERTICAL_RIGHT);
        for _ in 0..area.width - 2 {
            line.push_str(line::HORIZONTAL);
        }
        line.push_str(line::VERTICAL_LEFT);
        buf.set_string(area.left(), area.top(), line.clone(), self.border_style);
        buf.set_string(area.left(), area.bottom() - 1, line, self.border_style);

        // Title
        let offset = area.width / 2 - self.title.len() as u16 / 2;
        let title_x = area.left() + offset;
        let title_y = area.y;
        buf.set_string(title_x, title_y, self.title.clone(), self.title_style);

        // Title Tee's
        buf.set_string(
            title_x - 1,
            area.top(),
            line::VERTICAL_LEFT,
            self.border_style
        );
        buf.set_string(
            title_x + self.title.len() as u16,
            area.top(),
            line::VERTICAL_RIGHT,
            self.border_style
        );
    }
}

pub fn ui<B: Backend>(frame: &mut Frame<B>, app: &mut App, state: AppState) {
    match state {
        AppState::ProjectMenu => render_project_menu(frame, app),
        AppState::EditProject(prev) => {
            ui(frame, app, *prev);
            render_single_input_editor(frame, app, "Edit Project Details".to_string());
        },
        AppState::CreateProject(prev) => {
            ui(frame, app, *prev);
            render_single_input_editor(frame, app, "Create New Project".to_string());
        },
        AppState::DeleteProject(prev) => {
            ui(frame, app, *prev);
            render_prompt(frame, "Delete Highlighted Project?".to_string());
        },
        AppState::Tracker => render_tracker(frame, app),
        AppState::TaskView(prev) => {
            ui(frame, app, *prev);
            render_task_data(frame, app);
        },
        AppState::BacklogPopup(prev) => {
            ui(frame, app, *prev);
            render_list_popup(frame, app);
        },
        AppState::ArchivePopup(prev) => {
            ui(frame, app, *prev);
            render_list_popup(frame, app);
        },
        AppState::EditTask(prev) => {
            ui(frame, app, *prev);
            render_task_editor(frame, app, "Edit Task Details".to_string());
        },
        AppState::CreateTask(prev) => {
            ui(frame, app, *prev);
            render_task_editor(frame, app, "Create New Task".to_string());
        },
        AppState::DeleteTask(prev) => {
            ui(frame, app, *prev);
            render_prompt(frame, "Delete Highlighted Task?".to_string());
        },
        AppState::EditList(prev) => {
            ui(frame, app, *prev);
            render_single_input_editor(frame, app, "Edit List Details".to_string());
        },
        AppState::CreateList(prev) => {
            ui(frame, app, *prev);
            render_single_input_editor(frame, app, "Create New List".to_string());
        },
        AppState::DeleteList(prev) => {
            ui(frame, app, *prev);
            render_prompt(frame, "Delete Focused List?".to_string());
        },
    }
}

fn render_project_menu<B: Backend>(
    frame: &mut Frame<B>,
    app: &mut App
) {
    let size = frame.size();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
            Constraint::Length(9),
            Constraint::Min(10),
            Constraint::Length(3),
            ]
            .as_ref()
        )
        .split(size);

    let banner = raw_para!(
        "",
        "    __             __      _ ",
        "   / /______ _____/ /___ _(_)",
        "  / //_/ __ `/ __  / __ `/ / ",
        " / ,< / /_/ / /_/ / /_/ / /  ",
        "/_/|_|\\__,_/\\__,_/\\__,_/_/   ",
        "",
        "A Task Tracker For The Terminal",
        ""
    );

    let banner = Paragraph::new(banner)
        .block(Block::default())
        .style(
            Style::default()
            .fg(Color::Red)
            .add_modifier(Modifier::BOLD)
        )
        .alignment(Alignment::Center);

    frame.render_widget(banner, chunks[0]);

    let list_area = centered_rect(40, 100, chunks[1]);

    if app.project_list.is_empty() {
        let mut commands = raw_para!(
            "There are currently no projects.",
            "",
            "Hit 'n' to create and open a new project."
        );

        for _ in 0..chunks[1].height / 2 - 2 {
            commands.insert(0, Spans::from(Span::raw("")));
        }

        let commands = Paragraph::new(commands)
            .block(Block::default())
            .style(
                Style::default()
                .add_modifier(Modifier::BOLD)
            )
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true });

        frame.render_widget(commands, chunks[1]);
    } else {
        let highlight = Style::default()
            .add_modifier(Modifier::REVERSED);

        let container = CustomBorder::new()
            .title("Projects".to_string());

        frame.render_widget(container, list_area);

        let list_area = shrink_rect(list_area, 1);

        let items: Vec<ListItem> = app.project_list
            .projects
            .iter()
            .map(|p| {
                ListItem::new(
                    Span::raw(p)
                )
            })
        .collect();

        let list = List::new(items)
            .block(Block::default())
            .highlight_style(highlight);

        frame.render_stateful_widget(list, list_area, &mut app.project_list.state);
    }

    let info = raw_para!(
        "",
        "kadai v1.1.0 by Ben Buchanan (https://github.com/Nynergy)"
    );

    let info = Paragraph::new(info)
        .block(Block::default())
        .alignment(Alignment::Center);

    frame.render_widget(info, chunks[2]);
}

fn render_single_input_editor<B: Backend>(
    frame: &mut Frame<B>,
    app: &mut App,
    editor_title: String,
) {
    let size = frame.size();
    let area = centered_fixed_size_rect((size.width as f32 * 0.6) as usize, 7, size);
    let area_block = Block::default()
        .title(
            Span::styled(
                editor_title,
                Style::default()
                .add_modifier(Modifier::BOLD)
            )
        )
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_type(BorderType::Double);

    frame.render_widget(Clear, area); // Clear the area first
    frame.render_widget(area_block, area);

    let inner_area = shrink_rect(area, 1);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
            Constraint::Length(3),
            Constraint::Length(1),
            Constraint::Length(1),
            ]
            .as_ref()
        )
        .split(inner_area);

    let input = app.get_focused_input().clone();
    let name = Paragraph::new(&input)
        .style(
            Style::default().fg(Color::Red)
        )
        .block(
            Block::default()
            .borders(Borders::ALL)
            .title(input.name.clone())
        )
        .wrap(Wrap { trim: true });

    frame.render_widget(name, chunks[0]);

    // Display the blinking cursor, wrapped appropriately
    let input_area = chunks[0];
    let cursor_pos = get_wrapped_cursor_pos(&input, input_area);

    frame.set_cursor(
        chunks[0].x + cursor_pos.0 as u16 + 1,
        chunks[0].y + cursor_pos.1 as u16
    );

    let info = Paragraph::new(
        Span::styled(
            "Press Enter to Save Changes, Esc to Exit",
            Style::default()
            .fg(Color::Red)
            .add_modifier(Modifier::BOLD)
        ))
        .block(Block::default())
        .wrap(Wrap { trim: true })
        .alignment(Alignment::Center);

    frame.render_widget(info, chunks[1]);

    let info = Paragraph::new(
        Span::styled(
            "Press Delete to Clear Input",
            Style::default()
            .fg(Color::Red)
            .add_modifier(Modifier::BOLD)
        ))
        .block(Block::default())
        .wrap(Wrap { trim: true })
        .alignment(Alignment::Center);

    frame.render_widget(info, chunks[2]);
}


fn render_tracker<B: Backend>(
    frame: &mut Frame<B>,
    app: &mut App
) {
    let size = frame.size();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
            Constraint::Length(3),
            Constraint::Min(10),
            ]
            .as_ref()
        )
        .split(size);

    render_info_bar(frame, app, chunks[0]);

    let list_width = size.width / app.task_lists.len() as u16;

    let mut constraints: Vec<Constraint> = Vec::new();
    for _ in 0..app.task_lists.len() / 2 {
        constraints.push(Constraint::Length(list_width));
    }
    constraints.push(Constraint::Min(10));
    if app.task_lists.len() % 2 == 0 {
        for _ in 0..app.task_lists.len() / 2 - 1 {
            constraints.push(Constraint::Length(list_width));
        }
    } else {
        for _ in 0..app.task_lists.len() / 2 {
            constraints.push(Constraint::Length(list_width));
        }
    }

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(constraints.as_ref())
        .split(chunks[1]);

    for i in 0..app.task_lists.len() {
        render_task_list(frame, app, chunks[i], i);
    }
}

fn render_info_bar<B: Backend>(
    frame: &mut Frame<B>,
    app: &mut App,
    area: Rect
) {
    let block = Block::default()
        .borders(Borders::ALL);

    frame.render_widget(block, area);

    let inner_area = shrink_rect(area, 1);

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
            Constraint::Length(1),
            Constraint::Min(10),
            Constraint::Length(1),
            ]
            .as_ref()
        )
        .split(inner_area);

    let left = Spans::from(vec![
        Span::styled(
            "Project: ",
            Style::default()
            .fg(Color::Red)
            .add_modifier(Modifier::BOLD)
        ),
        Span::styled(
            &app.project_title,
            Style::default()
            .add_modifier(Modifier::BOLD)
        ),
    ]);

    let left = Paragraph::new(left)
        .block(Block::default())
        .wrap(Wrap { trim: true });

    frame.render_widget(left, chunks[1]);

    let right = Spans::from(vec![
        Span::styled(
            app.backlog.len().to_string(),
            Style::default()
            .fg(Color::Indexed(app.backlog.color_index))
            .add_modifier(Modifier::BOLD)
        ),
        Span::styled(
            " Backlogged",
            Style::default()
            .fg(Color::Indexed(app.backlog.color_index))
            .add_modifier(Modifier::BOLD)
        ),
        Span::raw(" | "),
        Span::styled(
            app.num_tracked_tasks().to_string(),
            Style::default()
            .add_modifier(Modifier::BOLD)
        ),
        Span::styled(
            " Tracked",
            Style::default()
            .add_modifier(Modifier::BOLD)
        ),
        Span::raw(" | "),
        Span::styled(
            app.archive.len().to_string(),
            Style::default()
            .fg(Color::Indexed(app.archive.color_index))
            .add_modifier(Modifier::BOLD)
        ),
        Span::styled(
            " Archived",
            Style::default()
            .fg(Color::Indexed(app.archive.color_index))
            .add_modifier(Modifier::BOLD)
        ),
    ]);

    let right = Paragraph::new(right)
        .block(Block::default())
        .alignment(Alignment::Right)
        .wrap(Wrap { trim: true });

    frame.render_widget(right, chunks[1]);

    if app.unsaved_changes {
        let middle = Spans::from(vec![
            Span::styled(
                "Unsaved Changes",
                Style::default()
                .add_modifier(Modifier::BOLD)
            ),
        ]);

        let middle = Paragraph::new(middle)
            .block(Block::default())
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true });

        frame.render_widget(middle, chunks[1]);
    }
}

fn render_task_data<B: Backend>(
    frame: &mut Frame<B>,
    app: &mut App
) {
    if let Some(task) = app.get_selected_task() {
        let size = frame.size();
        let area = centered_rect(60, 40, size);
        let area_block = Block::default()
            .title(
                Span::styled(
                    "Task Details",
                    Style::default()
                    .add_modifier(Modifier::BOLD)
                )
            )
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .border_type(BorderType::Double);

        frame.render_widget(Clear, area); // Clear the area first
        frame.render_widget(area_block, area);

        let inner_area = shrink_rect(area, 1);

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                Constraint::Min(2),
                Constraint::Length(1),
                ]
                .as_ref()
            )
            .split(inner_area);

        let description: String;
        match &task.description {
            Some(d) => description = d.to_string(),
            None => description = "N/A".to_string()
        }
        let category: String;
        match &task.category {
            Some(c) => category = c.to_string(),
            None => category = "N/A".to_string()
        }
        let details = vec![
            Spans::from(
                vec![
                    Span::styled(
                        "Summary: ",
                        Style::default()
                        .add_modifier(Modifier::BOLD)
                    ),
                    Span::raw(&task.summary),
                ]
            ),
            Spans::from(Span::raw("")),
            Spans::from(
                vec![
                    Span::styled(
                        "Category: ",
                        Style::default()
                        .add_modifier(Modifier::BOLD)
                    ),
                    Span::raw(category),
                ]
            ),
            Spans::from(Span::raw("")),
            Spans::from(
                vec![
                    Span::styled(
                        "Description: ",
                        Style::default()
                        .add_modifier(Modifier::BOLD)
                    ),
                    Span::raw(description),
                ]
            ),
        ];
        let details = Paragraph::new(details)
            .block(Block::default())
            .wrap(Wrap { trim: true })
            .scroll((app.detail_scroll, 0));

        frame.render_widget(details, chunks[0]);

        let info = Paragraph::new(
            Span::styled(
                "Press Enter to close, 'j' and 'k' to Scroll",
                Style::default()
                .fg(Color::Red)
                .add_modifier(Modifier::BOLD)
            ))
            .block(Block::default())
            .wrap(Wrap { trim: true })
            .alignment(Alignment::Center);

        frame.render_widget(info, chunks[1]);
    }
}

fn render_list_popup<B: Backend>(
    frame: &mut Frame<B>,
    app: &mut App,
) {
    let size = frame.size();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
            Constraint::Percentage(50),
            Constraint::Percentage(50),
            ]
            .as_ref()
        )
        .split(size);

    let task_list = app.get_mut_focused_list(&app.state.clone());
    let container = CustomBorder::new()
        .title(task_list.name.clone())
        .title_style(
            Style::default()
            .fg(
                Color::Indexed(
                    task_list.color_index
                )
            )
            .add_modifier(Modifier::BOLD)
        )
        .border_style(
            Style::default()
            .fg(
                Color::Indexed(
                    task_list.color_index
                )
            )
        );

    frame.render_widget(Clear, chunks[1]); // Clear the area first
    frame.render_widget(container, chunks[1]);

    let items: Vec<ListItem> = task_list
        .tasks
        .iter()
        .map(|i| {
            ListItem::new(task_spans(i, chunks[1].width - 2))
        })
        .collect();

    let list = List::new(items)
        .block(Block::default())
        .highlight_style(
            Style::default()
            .add_modifier(Modifier::REVERSED)
        );

    let inner_area = shrink_rect(chunks[1], 1);

    frame.render_stateful_widget(list, inner_area, &mut task_list.state);
}

fn render_task_editor<B: Backend>(
    frame: &mut Frame<B>,
    app: &mut App,
    editor_title: String,
) {
    let size = frame.size();
    let area = centered_rect(60, 40, size);
    let area_block = Block::default()
        .title(
            Span::styled(
                editor_title,
                Style::default()
                .add_modifier(Modifier::BOLD)
            )
        )
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_type(BorderType::Double);

    frame.render_widget(Clear, area); // Clear the area first
    frame.render_widget(area_block, area);

    let inner_area = shrink_rect(area, 1);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
            Constraint::Length(3),
            Constraint::Min(3),
            Constraint::Length(3),
            Constraint::Length(1),
            Constraint::Length(1),
            ]
            .as_ref()
        )
        .split(inner_area);

    for i in 0..app.task_detail_inputs.len() {
        let input = &app.task_detail_inputs[i];
        let field = Paragraph::new(input.clone())
            .style(
                if app.active_detail_input == i {
                    Style::default().fg(Color::Red)
                } else {
                    Style::default()
                }
            )
            .block(
                Block::default()
                .borders(Borders::ALL)
                .title(input.name.clone())
            )
            .wrap(Wrap { trim: true });

        frame.render_widget(field, chunks[i]);
    }

    // Display the blinking cursor, wrapped appropriately
    let i = app.active_detail_input;
    let input = &app.task_detail_inputs[i];
    let input_area = chunks[i];
    let cursor_pos = get_wrapped_cursor_pos(input, input_area);

    frame.set_cursor(
        chunks[i].x + cursor_pos.0 as u16 + 1,
        chunks[i].y + cursor_pos.1 as u16
    );

    let info = Paragraph::new(
        Span::styled(
            "Press Enter to Save Changes, Esc to Exit",
            Style::default()
            .fg(Color::Red)
            .add_modifier(Modifier::BOLD)
        ))
        .block(Block::default())
        .wrap(Wrap { trim: true })
        .alignment(Alignment::Center);

    frame.render_widget(info, chunks[3]);

    let info = Paragraph::new(
        Span::styled(
            "Press Tab to Cycle Focus, Delete to Clear Input",
            Style::default()
            .fg(Color::Red)
            .add_modifier(Modifier::BOLD)
        ))
        .block(Block::default())
        .wrap(Wrap { trim: true })
        .alignment(Alignment::Center);

    frame.render_widget(info, chunks[4]);
}

fn render_prompt<B: Backend>(
    frame: &mut Frame<B>,
    prompt: String,
) {
    let size = frame.size();
    let area = centered_fixed_size_rect(prompt.len() + 6, 7, size);
    let area_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Double);

    frame.render_widget(Clear, area); // Clear the area first
    frame.render_widget(area_block, area);

    let inner_area = shrink_rect(area, 1);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
            Constraint::Length(1),
            ]
            .as_ref()
        )
        .split(inner_area);

    let text = vec![
        Spans::from(
            Span::raw("")
        ),
        Spans::from(
            Span::styled(
                prompt,
                Style::default()
                .add_modifier(Modifier::BOLD)
            )
        ),
        Spans::from(
            Span::raw("")
        ),
        Spans::from(
            Span::styled(
                "(Y)es (N)o",
                Style::default()
                .add_modifier(Modifier::BOLD)
            )
        ),
        ];
    let text = Paragraph::new(text)
        .block(Block::default())
        .wrap(Wrap { trim: true })
        .alignment(Alignment::Center);

    frame.render_widget(text, chunks[0]);
}

fn render_task_list<B: Backend>(
    frame: &mut Frame<B>,
    app: &mut App,
    chunk: Rect,
    list_num: usize
) {
    let highlight: Style;
    let border: Style;

    let items: Vec<ListItem> = app
        .task_lists[list_num]
        .tasks
        .iter()
        .map(|i| {
            ListItem::new(task_spans(i, chunk.width - 2))
        })
        .collect();

    if app.active_list == list_num {
        highlight = Style::default()
            .add_modifier(Modifier::REVERSED);
        border = Style::default()
            .fg(Color::Indexed(app.task_lists[list_num].color_index))
            .add_modifier(Modifier::BOLD);
    } else {
        highlight = Style::default();
        border = Style::default();
    }

    let container = CustomBorder::new()
        .title(app.task_lists[list_num].name.clone())
        .title_style(
            Style::default()
            .fg(
                Color::Indexed(
                    app.task_lists[list_num].color_index
                )
            )
        )
        .border_style(border);

    frame.render_widget(container, chunk);

    let list = List::new(items)
        .block(Block::default())
        .highlight_style(highlight);

    let inner_area = shrink_rect(chunk, 1);

    frame.render_stateful_widget(list, inner_area, &mut app.task_lists[list_num].state);
}

fn task_spans<'a>(task: &Task, width: u16) -> Vec<Spans<'a>> {
    let mut lines = Vec::new();

    create_top_line(&mut lines, width);
    create_summary_and_category_line(&mut lines, width, task);
    create_description_lines(&mut lines, width, task);
    create_bottom_line(&mut lines, width);

    lines
}

fn create_top_line(lines: &mut Vec<Spans>, width: u16) {
    let mut line = String::from(line::TOP_LEFT);
    for _ in 0..width - 2 {
        line.push_str(line::HORIZONTAL);
    }
    line.push_str(line::TOP_RIGHT);
    lines.push(Spans::from(line));
}

fn create_summary_and_category_line(lines: &mut Vec<Spans>, width: u16, task: &Task) {
    let line_style = Style::default()
        .add_modifier(Modifier::BOLD | Modifier::UNDERLINED);

    // Summary Left Side
    let line = String::from(
        format!("{} ", line::VERTICAL)
    );
    let mut spans = vec![Span::raw(line)];

    // Summary Text
    let mut summary = task.summary.clone();
    if task.summary.len() >= width as usize / 3 * 2 {
        summary.truncate(width as usize / 3 * 2 - 5);
        summary = format!("{}...", summary);
    }
    let line = String::from(&summary);
    spans.push(Span::styled(line, line_style));

    // Category Text
    if let Some(category) = &task.category {
        let mut category = category.clone();
        if category.len() >= width as usize / 3 {
            category.truncate(width as usize / 3 - 5);
            category = format!("{}...", category);
        }
        let line = String::from(&category);
        spans.push(Span::styled(line, line_style));
    }

    // Space Between Summary and Category
    let current_width = spans
        .iter()
        .map(|span| span.width())
        .sum::<usize>();
    let remaining_width = (width - 2) as usize - current_width;

    let mut line = String::new();
    for _ in 0..remaining_width {
        line.push_str(" ");
    }
    let index = cmp::max(2, spans.len() - 1);
    spans.insert(index, Span::styled(line, line_style));

    // Category Right Side
    let line = String::from(
        format!(" {}", line::VERTICAL)
    );
    spans.push(Span::raw(line));
    lines.push(Spans::from(spans));
}

fn create_description_lines(lines: &mut Vec<Spans>, width: u16, task: &Task) {
    if let Some(description) = &task.description {
        let mut wrapped = wrap(description, (width - 4) as usize);
        if wrapped.len() > 3 {
            wrapped.truncate(2);
            wrapped.push(Cow::Borrowed("..."));
        }

        for l in wrapped {
            // Description Left Side
            let mut spans = vec![
                Span::raw(
                    String::from(format!("{} ", line::VERTICAL))
                )
            ];

            // Description Text
            spans.push(Span::raw(String::from(l)));

            // Description Right Side
            let current_width = spans
                .iter()
                .map(|span| span.width())
                .sum::<usize>();
            let remaining_width = (width - 1) as usize - current_width;

            let mut line = String::new();
            for _ in 0..remaining_width {
                line.push_str(" ");
            }
            line.push_str(line::VERTICAL);
            spans.push(Span::raw(line));
            lines.push(Spans::from(spans));
        }
    }
}

fn create_bottom_line(lines: &mut Vec<Spans>, width: u16) {
    let mut line = String::from(line::BOTTOM_LEFT);
    for _ in 0..width - 2 {
        line.push_str(line::HORIZONTAL);
    }
    line.push_str(line::BOTTOM_RIGHT);
    lines.push(Spans::from(line));
}

fn centered_rect(percent_x: usize, percent_y: usize, size: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) as u16 / 2),
                Constraint::Percentage(percent_y as u16),
                Constraint::Percentage((100 - percent_y) as u16 / 2),
            ]
            .as_ref(),
        )
        .split(size);

    let popup_rect = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) as u16 / 2),
                Constraint::Percentage(percent_x as u16),
                Constraint::Percentage((100 - percent_x) as u16 / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1];

    popup_rect
}

fn centered_fixed_size_rect(width: usize, height: usize, size: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(size.height / 2 - height as u16 / 2),
                Constraint::Min(height as u16),
                Constraint::Length(size.height / 2 - height as u16 / 2),
            ]
            .as_ref(),
        )
        .split(size);

    let popup_rect = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Length(size.width / 2 - width as u16 / 2),
                Constraint::Min(width as u16),
                Constraint::Length(size.width / 2 - width as u16 / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1];

    popup_rect
}

fn shrink_rect(rect: Rect, amount: u16) -> Rect {
    let margin = Margin { vertical: amount, horizontal: amount };
    rect.inner(&margin)
}

fn get_wrapped_cursor_pos(input: &Input, area: Rect) -> (usize, usize) {
    let input_width = area.width as usize - 2;
    let trailing_spaces = &input.num_trailing_spaces();
    let wrap_options = Options::new(input_width)
        .wrap_algorithm(WrapAlgorithm::FirstFit);
    let strings = wrap(&input.text, wrap_options);
    let string = &mut strings[strings.len() - 1].to_string();
    for _ in 0..*trailing_spaces {
        string.push(' ');
    }

    (string.len(), strings.len())
}
