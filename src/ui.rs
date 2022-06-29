use std::borrow::Cow;
use textwrap::wrap;
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
use crate::task_list::*;

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
        AppState::Tracker => {
            render_tracker(frame, app);
        },
        AppState::TaskView(prev) => {
            ui(frame, app, *prev);
            render_task_data(frame, app);
        },
        AppState::BacklogPopup(prev) => {
            ui(frame, app, *prev);
            render_backlog_popup(frame, app);
        },
        AppState::ArchivePopup(prev) => {
            ui(frame, app, *prev);
            render_archive_popup(frame, app);
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
            render_prompt(frame, app, "Delete Highlighted Task?".to_string());
        },
        AppState::EditList(prev) => {
            ui(frame, app, *prev);
            render_list_editor(frame, app, "Edit List Details".to_string());
        },
        AppState::CreateList(prev) => {
            ui(frame, app, *prev);
            render_list_editor(frame, app, "Create New List".to_string());
        },
        AppState::DeleteList(prev) => {
            ui(frame, app, *prev);
            render_prompt(frame, app, "Delete Focused List?".to_string());
        },
    }
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
            app.backlog.tasks.len().to_string(),
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
            app.archive.tasks.len().to_string(),
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

fn render_backlog_popup<B: Backend>(
    frame: &mut Frame<B>,
    app: &mut App
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

    let container = CustomBorder::new()
        .title(app.backlog.name.clone())
        .title_style(
            Style::default()
            .fg(
                Color::Indexed(
                    app.backlog.color_index
                )
            )
            .add_modifier(Modifier::BOLD)
        )
        .border_style(
            Style::default()
            .fg(
                Color::Indexed(
                    app.backlog.color_index
                )
            )
        );

    frame.render_widget(Clear, chunks[1]); // Clear the area first
    frame.render_widget(container, chunks[1]);

    let items: Vec<ListItem> = app
        .backlog
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

    frame.render_stateful_widget(list, inner_area, &mut app.backlog.state);
}

fn render_archive_popup<B: Backend>(
    frame: &mut Frame<B>,
    app: &mut App
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

    let container = CustomBorder::new()
        .title(app.archive.name.clone())
        .title_style(
            Style::default()
            .fg(
                Color::Indexed(
                    app.archive.color_index
                )
            )
            .add_modifier(Modifier::BOLD)
        )
        .border_style(
            Style::default()
            .fg(
                Color::Indexed(
                    app.archive.color_index
                )
            )
        );

    frame.render_widget(Clear, chunks[1]); // Clear the area first
    frame.render_widget(container, chunks[1]);

    let items: Vec<ListItem> = app
        .archive
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

    frame.render_stateful_widget(list, inner_area, &mut app.archive.state);
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

    let summary = Paragraph::new(app.task_detail_inputs[0].clone())
        .style(
            if app.active_detail_input == 0 {
                Style::default().fg(Color::Red)
            } else {
                Style::default()
            }
        )
        .block(
            Block::default()
            .borders(Borders::ALL)
            .title("Summary")
        )
        .wrap(Wrap { trim: true });

    frame.render_widget(summary, chunks[0]);

    let description = Paragraph::new(app.task_detail_inputs[1].clone())
        .style(
            if app.active_detail_input == 1 {
                Style::default().fg(Color::Red)
            } else {
                Style::default()
            }
        )
        .block(
            Block::default()
            .borders(Borders::ALL)
            .title("Description")
        )
        .wrap(Wrap { trim: true });

    frame.render_widget(description, chunks[1]);

    let category = Paragraph::new(app.task_detail_inputs[2].clone())
        .style(
            if app.active_detail_input == 2 {
                Style::default().fg(Color::Red)
            } else {
                Style::default()
            }
        )
        .block(
            Block::default()
            .borders(Borders::ALL)
            .title("Category")
        )
        .wrap(Wrap { trim: true });

    frame.render_widget(category, chunks[2]);

    // Display the blinking cursor, wrapped appropriately
    let i = app.active_detail_input;
    let input = &app.task_detail_inputs[i];
    let input_width = chunks[i].width as usize - 2;
    let trailing_spaces = &input.num_trailing_spaces();
    let strings = wrap(&input.text, input_width);
    let string = &mut strings[strings.len() - 1].to_string();
    for _ in 0..*trailing_spaces {
        string.push(' ');
    }

    let mut cursor_pos = (0, 0);
    let mut running_pos = 0;
    for line in strings.iter() {
        let new_len = line.len() + running_pos + trailing_spaces;
        if new_len < input.pos {
            running_pos += line.len();
            if line.len() < input_width {
                running_pos += 1;
            }
            cursor_pos.1 += 1;
        } else {
            cursor_pos.0 = input.pos - running_pos;
            break;
        }
    }

    frame.set_cursor(
        chunks[i].x + cursor_pos.0 as u16 + 1,
        chunks[i].y + cursor_pos.1 as u16 + 1
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

fn render_list_editor<B: Backend>(
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

    let name = Paragraph::new(app.list_detail_input.clone())
        .style(
            Style::default().fg(Color::Red)
        )
        .block(
            Block::default()
            .borders(Borders::ALL)
            .title("List Name")
        )
        .wrap(Wrap { trim: true });

    frame.render_widget(name, chunks[0]);

    // Display the blinking cursor, wrapped appropriately
    let input = &app.list_detail_input;
    let input_width = chunks[0].width as usize - 2;
    let trailing_spaces = &input.num_trailing_spaces();
    let strings = wrap(&input.text, input_width);
    let string = &mut strings[strings.len() - 1].to_string();
    for _ in 0..*trailing_spaces {
        string.push(' ');
    }

    let mut cursor_pos = (0, 0);
    let mut running_pos = 0;
    for line in strings.iter() {
        let new_len = line.len() + running_pos + trailing_spaces;
        if new_len < input.pos {
            running_pos += line.len();
            if line.len() < input_width {
                running_pos += 1;
            }
            cursor_pos.1 += 1;
        } else {
            cursor_pos.0 = input.pos - running_pos;
            break;
        }
    }

    frame.set_cursor(
        chunks[0].x + cursor_pos.0 as u16 + 1,
        chunks[0].y + cursor_pos.1 as u16 + 1
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

fn render_prompt<B: Backend>(
    frame: &mut Frame<B>,
    _app: &mut App,
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
    let mut lines: Vec<Spans> = Vec::new();

    // Top Line
    let mut line = String::new();
    line.push_str(line::TOP_LEFT);
    for _ in 0..width - 2 {
        line.push_str(line::HORIZONTAL);
    }
    line.push_str(line::TOP_RIGHT);
    lines.push(Spans::from(line));

    // Summary Left Side
    let mut line = String::new();
    let mut spans: Vec<Span> = Vec::new();
    line.push_str(&format!("{} ", line::VERTICAL));
    spans.push(Span::raw(line));

    // Summary Text
    let mut summary = task.summary.clone();
    if task.summary.len() >= width as usize / 3 * 2 {
        summary.truncate(width as usize / 3 * 2 - 5);
        summary = format!("{}...", summary);
    }
    let mut line = String::new();
    line.push_str(&summary);
    spans.push(
        Span::styled(
            line,
            Style::default()
            .add_modifier(Modifier::BOLD | Modifier::UNDERLINED)
        )
    );

    // Category Text
    if let Some(category) = &task.category {
        let mut category = category.clone();
        if category.len() >= width as usize / 3 {
            category.truncate(width as usize / 3 - 5);
            category = format!("{}...", category);
        }
        let mut line = String::new();
        line.push_str(&category);
        spans.push(
            Span::styled(
                line,
                Style::default()
                .add_modifier(Modifier::BOLD | Modifier::UNDERLINED)
            )
        );
    }

    // Summary Right Side
    let remaining_width = (width - 2) as usize - spans.iter()
                                                      .map(|span| span.width())
                                                      .sum::<usize>();

    let mut line = String::new();
    for _ in 0..remaining_width {
        line.push_str(" ");
    }
    match task.category {
        Some(_) => spans.insert(
            spans.len() - 1, Span::styled(
                line,
                Style::default()
                .add_modifier(Modifier::BOLD | Modifier::UNDERLINED),
                )
            ),
        None => spans.push(
            Span::styled(
                line,
                Style::default()
                .add_modifier(Modifier::BOLD | Modifier::UNDERLINED),
                )
            ),
    }

    let mut line = String::new();
    line.push_str(&format!(" {}", line::VERTICAL));
    spans.push(Span::raw(line));
    lines.push(Spans::from(spans));

    // Description
    if let Some(description) = &task.description {
        let mut wrapped = wrap(description, (width - 4) as usize);
        if wrapped.len() > 3 {
            wrapped.truncate(2);
            wrapped.push(Cow::Borrowed("..."));
        }

        for l in wrapped {
            // Description Left Side
            let mut line = String::new();
            let mut spans: Vec<Span> = Vec::new();
            line.push_str(&format!("{} ", line::VERTICAL));
            spans.push(Span::raw(line));

            // Description Text
            let mut line = String::new();
            line.push_str(&l);
            spans.push(Span::raw(line));

            // Description Right Side
            let remaining_width = (width - 1) as usize - spans.iter()
                                                              .map(|span| span.width())
                                                              .sum::<usize>();
            let mut line = String::new();
            for _ in 0..remaining_width {
                line.push_str(" ");
            }
            line.push_str(line::VERTICAL);
            spans.push(Span::raw(line));
            lines.push(Spans::from(spans));
        }
    }

    // Bottom Line
    let mut line = String::new();
    line.push_str(line::BOTTOM_LEFT);
    for _ in 0..width - 2 {
        line.push_str(line::HORIZONTAL);
    }
    line.push_str(line::BOTTOM_RIGHT);
    lines.push(Spans::from(line));

    lines
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
