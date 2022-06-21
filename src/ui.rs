use std::borrow::Cow;
use textwrap::wrap;
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Margin, Rect},
    style::{Color, Modifier, Style},
    symbols::line,
    text::{Span, Spans},
    widgets::{Block, Borders, BorderType, Clear, List, ListItem, Paragraph, Wrap},
    Frame,
};

use crate::app::*;
use crate::task_list::*;

pub fn ui<B: Backend>(frame: &mut Frame<B>, app: &mut App) {
    match app.state {
        AppState::Tracker => {
            render_tracker(frame, app);
        },
        AppState::TaskView => {
            render_tracker(frame, app);
            render_task_data(frame, app);
        }
    }
}

fn render_tracker<B: Backend>(
    frame: &mut Frame<B>,
    app: &mut App
) {
    let size = frame.size();
    let third = size.width / 3;

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
            Constraint::Length(third),
            Constraint::Min(10),
            Constraint::Length(third),
            ]
            .as_ref()
        )
        .split(size);

    let list_styles = vec![
        ("Planned", Color::Magenta),
        ("In Progress", Color::Yellow),
        ("Completed", Color::Green),
    ];

    for i in 0..list_styles.len() {
        render_task_list(frame, app, list_styles[i], chunks[i], i);
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
                Constraint::Min(10),
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
                "Press ESC to close",
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

fn render_task_list<B: Backend>(
    frame: &mut Frame<B>,
    app: &mut App,
    list_style: (&str, Color),
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
            .fg(list_style.1)
            .add_modifier(Modifier::BOLD);
    } else {
        highlight = Style::default();
        border = Style::default();
    }

    let list = List::new(items)
        .block(
            Block::default()
            .title(
                Span::styled(
                    list_style.0,
                    Style::default().fg(list_style.1)
                )
            )
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .border_style(border)
        )
        .highlight_style(highlight);

    frame.render_stateful_widget(list, chunk, &mut app.task_lists[list_num].state);
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

fn shrink_rect(rect: Rect, amount: u16) -> Rect {
    let margin = Margin { vertical: amount, horizontal: amount };
    rect.inner(&margin)
}
