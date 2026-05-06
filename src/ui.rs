use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Stylize},
    widgets::{Block, Paragraph, Wrap},
};

use crate::app::App;

pub fn ui(frame: &mut Frame, app: &App) {
    let centered = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Fill(1),
            Constraint::Max(120),
            Constraint::Fill(1),
        ])
        .split(frame.area());

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(2), Constraint::Min(1)])
        .split(centered[1]);

    let block = Block::default()
        .title("rubberducky")
        .title_alignment(Alignment::Center);

    let text = format!("{:?}\n{}", app.mode, app.query);

    let mode = Paragraph::new(text)
        .block(block)
        .fg(Color::default())
        .centered();

    frame.render_widget(mode, chunks[0]);

    let user_input = Paragraph::new(app.query.as_str())
        .block(Block::new())
        .wrap(Wrap { trim: true });

    frame.render_widget(user_input, chunks[1]);
}
