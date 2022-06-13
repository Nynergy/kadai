use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    text::Span,
    widgets::{Block, Borders},
    Frame,
};

use crate::app::*;

pub fn ui<B: Backend>(frame: &mut Frame<B>, _app: &App) {
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

    let block = Block::default()
        .title(
            Span::styled(
                "Planned",
                Style::default().fg(Color::Magenta)
            )
        )
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL);
    frame.render_widget(block, chunks[0]);

    let block = Block::default()
        .title(
            Span::styled(
                "In Progess",
                Style::default().fg(Color::Yellow)
            )
        )
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL);
    frame.render_widget(block, chunks[1]);

    let block = Block::default()
        .title(
            Span::styled(
                "Completed",
                Style::default().fg(Color::Green)
            )
        )
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL);
    frame.render_widget(block, chunks[2]);
}
