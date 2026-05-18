use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout},
    widgets::Block,
};

use crate::app::App;

pub fn ui(frame: &mut Frame, app: &mut App) {
    let centered = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Fill(1),
            Constraint::Max(120),
            Constraint::Fill(1),
        ])
        .split(frame.area());

    let line_count = app.user_input.num_lines() as u16;
    let min_height = 5;
    let max_height = 12;
    let height = line_count.clamp(min_height, max_height);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2),
            Constraint::Fill(1),
            Constraint::Length(height),
            Constraint::Length(1),
        ])
        .split(centered[1]);

    let block = Block::default()
        .title("rubberducky")
        .title_alignment(Alignment::Center);

    frame.render_widget(block, chunks[0]);

    let constraints: Vec<Constraint> = app
        .messages
        .iter()
        .map(|m| Constraint::Length((m.num_lines() + 2) as u16))
        .collect();
    let message_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
        .split(chunks[1]);
    for (i, message) in app.messages.iter_mut().enumerate() {
        message.render(frame, message_chunks[i]);
    }

    app.user_input.render(frame, chunks[2]);
    frame.render_widget(&app.mode, chunks[3]);
}
