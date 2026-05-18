use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Padding, Paragraph},
};

use crate::components::message::{Message, State};
use crate::constants::symbols::caret;

impl<'a> Message<'a> {
    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        let block = match self.state {
            State::FocusedOutput => Block::default().padding(Padding::uniform(1)),
            State::UnfocusedOutput => Block::default().padding(Padding::uniform(1)),
            State::FocusedInput => Block::default()
                .style(Style::default().bg(Color::Black))
                .padding(Padding::uniform(1)),
            State::UnfocusedInput => Block::default()
                .style(Style::default().bg(Color::DarkGray))
                .padding(Padding::uniform(1)),
        };

        let caret = match self.state {
            State::FocusedOutput => Paragraph::new(caret::OUTPUT_PENDING)
                .block(Block::default().padding(Padding::uniform(1))),
            State::UnfocusedOutput => {
                Paragraph::new(caret::OUTPUT).block(Block::default().padding(Padding::uniform(1)))
            }
            State::FocusedInput => Paragraph::new(caret::INPUT_PENDING).block(
                Block::default()
                    .style(Style::default().bg(Color::Black))
                    .padding(Padding::uniform(1)),
            ),
            State::UnfocusedInput => Paragraph::new(caret::INPUT).block(
                Block::default()
                    .style(Style::default().bg(Color::DarkGray))
                    .padding(Padding::uniform(1)),
            ),
        };

        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Length(3), Constraint::Fill(1)])
            .split(area);

        frame.render_widget(caret, chunks[0]);
        self.textarea.set_block(block);
        frame.render_widget(&self.textarea, chunks[1]);
    }
}
