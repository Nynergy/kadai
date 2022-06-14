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
        .list_items[list_num]
        .iter()
        .map(|i| {
            ListItem::new(task_spans(i.clone(), chunk.width - 2))
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

    frame.render_stateful_widget(list, chunk, &mut app.list_states[list_num]);
}

fn task_spans<'a>(task: String, width: u16) -> Vec<Spans<'a>> {
    let mut spans: Vec<Spans> = Vec::new();

    let mut line = String::new();
    line.push_str(line::TOP_LEFT);
    for _ in 0..width - 2 {
        line.push_str(line::HORIZONTAL);
    }
    line.push_str(line::TOP_RIGHT);
    spans.push(Spans::from(line));

    let mut line = String::new();
    line.push_str(&format!("{} {}", line::VERTICAL, task));
    let remaining_width = width - (line.len() as u16) + 1;
    for _ in 0..remaining_width {
        line.push_str(" ");
    }
    line.push_str(line::VERTICAL);
    spans.push(Spans::from(line));

    let mut line = String::new();
    line.push_str(line::BOTTOM_LEFT);
    for _ in 0..width - 2 {
        line.push_str(line::HORIZONTAL);
    }
    line.push_str(line::BOTTOM_RIGHT);
    spans.push(Spans::from(line));

    spans
}
