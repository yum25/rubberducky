use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Paragraph, Widget},
};
use std::fmt;

use crate::app::Mode;
use crate::constants::symbols::powerline;

impl Widget for &Mode {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.line().render(area, buf);
    }
}

impl Mode {
    pub fn line<'a>(&self) -> Paragraph<'a> {
        let help = match self {
            Self::Normal => "type q to quit, type i to enter insert mode",
            Self::Replace(_) | Self::Insert => "type Esc to back to normal mode",
            Self::Visual => "type y to yank, type d to delete, type Esc to back to normal mode",
            Self::Operator(_) => "move cursor to apply operator",
        };

        let line = Line::from(vec![
            Span::styled(
                format!(" {} ", self),
                Style::new().fg(Color::Black).bg(self.color()).bold(),
            ),
            Span::styled(powerline::RIGHT, Style::new().fg(self.color())),
        ]);

        Paragraph::new(line)
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
