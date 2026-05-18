use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    symbols::border::Set,
    widgets::{Block, Borders},
};

use crate::components::message::{Message, State};

const TAPERED_BORDER: Set = Set {
    top_left: "▖",
    bottom_left: "▘",
    top_right: " ",
    bottom_right: " ",
    vertical_left: "▌",
    vertical_right: " ",
    horizontal_top: " ",
    horizontal_bottom: " ",
};

impl<'a> Message<'a> {
    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        let block = match self.state {
            State::FocusedOutput => Block::default()
                .borders(Borders::ALL)
                .border_set(TAPERED_BORDER)
                .border_style(Style::default().fg(Color::Green)),
            State::UnfocusedOutput => Block::default()
                .borders(Borders::ALL)
                .border_set(TAPERED_BORDER)
                .border_style(Style::default().fg(Color::White)),
            State::FocusedInput => Block::default().style(Style::default().bg(Color::Black)),
            State::UnfocusedInput => Block::default().style(Style::default().bg(Color::DarkGray)),
        };

        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Length(1),
                Constraint::Fill(1),
                Constraint::Length(1),
            ])
            .split(area);

        self.textarea.set_block(block);
        frame.render_widget(&self.textarea, chunks[1]);
    }
}
