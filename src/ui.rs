use std::borrow::Cow;
use textwrap::wrap;
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    symbols::line,
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem},
    Frame,
};

use crate::app::*;
use crate::task_list::*;

pub fn ui<B: Backend>(frame: &mut Frame<B>, app: &mut App) {
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
