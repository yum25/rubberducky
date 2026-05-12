use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout},
    widgets::{Block, Borders},
};
use std::fmt;

use crate::app::App;
use crate::app::Mode;

impl Mode {
    fn block<'a>(&self) -> Block<'a> {
        let help = match self {
            Self::Normal => "type q to quit, type i to enter insert mode",
            Self::Replace(_) | Self::Insert => "type Esc to back to normal mode",
            Self::Visual => "type y to yank, type d to delete, type Esc to back to normal mode",
            Self::Operator(_) => "move cursor to apply operator",
        };
        let title = format!("{} MODE ({})", self, help);
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
        .constraints([
            Constraint::Length(2),
            Constraint::Min(1),
            Constraint::Fill(1),
            Constraint::Length(1),
        ])
        .split(centered[1]);

    let block = Block::default()
        .title("rubberducky")
        .title_alignment(Alignment::Center);

    frame.render_widget(block, chunks[0]);

    frame.render_widget(&app.textarea, chunks[1]);
    frame.render_widget(app.mode.block(), chunks[3]);
}
