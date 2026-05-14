use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    symbols::border::Set,
    widgets::{Block, Borders},
};
use std::fmt;

use crate::app::App;
use crate::app::Mode;
use crate::components::message::Message;

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

impl Mode {
    fn block<'a>(&self) -> Block<'a> {
        let help = match self {
            Self::Normal => "type q to quit, type i to enter insert mode",
            Self::Replace(_) | Self::Insert => "type Esc to back to normal mode",
            Self::Visual => "type y to yank, type d to delete, type Esc to back to normal mode",
            Self::Operator(_) => "move cursor to apply operator",
        };
        let title = format!("{} MODE {}", self, help);
        Block::default().borders(Borders::ALL).title(title)
    }
}

impl fmt::Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Self::Normal => write!(f, "NORMAL"),
            Self::Insert => write!(f, "INSERT"),
            Self::Replace(_) => write!(f, "REPLACE"),
            Self::Visual => write!(f, "VISUAL"),
            Self::Operator(c) => write!(f, "O-PENDING({})", c),
        }
    }
}

impl<'a> Message<'a> {
    pub fn render(&mut self, frame: &mut Frame, area: Rect, focused: bool) {
        let block = if focused {
            Block::default()
                .borders(Borders::ALL)
                .border_set(TAPERED_BORDER)
                .border_style(Style::default().fg(Color::Green))
        } else {
            Block::default()
                .borders(Borders::ALL)
                .border_set(TAPERED_BORDER)
                .border_style(Style::default().fg(Color::White))
        };

        self.textarea.set_block(block);
        frame.render_widget(&self.textarea, area);
    }

    pub fn render_input(&mut self, frame: &mut Frame, area: Rect) {
        let block = Block::default().borders(Borders::ALL);

        self.textarea.set_block(block);
        frame.render_widget(&self.textarea, area);
    }
}

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
        message.render(frame, message_chunks[i], false);
    }

    app.user_input.render_input(frame, chunks[2]);
    frame.render_widget(app.mode.block(), chunks[3]);
}
