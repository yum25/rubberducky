use crate::event::AppEvent;
use ratatui::style::Style;
use ratatui_textarea::{CursorMove, Input, Key, TextArea};

use crate::app::Mode;

#[derive(Debug)]
enum Access {
    ReadOnly,
    Writeable,
}

#[derive(Debug)]
pub enum State {
    FocusedInput,
    UnfocusedInput,
    FocusedOutput,
    UnfocusedOutput,
}

#[derive(Debug)]
pub struct Message<'a> {
    pub textarea: TextArea<'a>,
    pub state: State,
    access: Access,
}

impl<'a> Default for Message<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> Message<'a> {
    pub fn new() -> Self {
        let mut textarea = TextArea::default();
        textarea.set_wrap_mode(ratatui_textarea::WrapMode::Word);
        textarea.set_cursor_line_style(Style::default());

        Self {
            textarea,
            access: Access::ReadOnly,
            state: State::FocusedInput,
        }
    }

    pub fn num_lines(&self) -> usize {
        self.textarea.lines().len()
    }
    pub fn set_cursor_style(&mut self, style: Style) {
        self.textarea.set_cursor_style(style);
    }

    pub fn clear_cursor_style(&mut self) {
        self.textarea.set_cursor_style(Style::default());
    }

    pub fn start_selection(&mut self) {
        self.textarea.start_selection();
    }

    pub fn cancel_selection(&mut self) {
        self.textarea.cancel_selection();
    }

    pub fn revoke_write_access(&mut self) {
        self.access = Access::ReadOnly;
    }

    pub fn grant_write_access(&mut self) {
        self.access = Access::Writeable;
    }

    pub fn focus(&mut self) {
        match self.state {
            State::UnfocusedInput => self.state = State::FocusedInput,
            State::UnfocusedOutput => self.state = State::FocusedOutput,
            _ => {}
        }
    }

    pub fn unfocus(&mut self) {
        match self.state {
            State::FocusedInput => self.state = State::UnfocusedInput,
            State::FocusedOutput => self.state = State::UnfocusedOutput,
            _ => {}
        }
    }

    pub fn handle_key_event(&mut self, mode: &Mode, input: Input) -> AppEvent {
        match mode {
            Mode::Normal => self.handle_normal_mode(input),
            Mode::Insert => self.handle_insert_mode(input),
            Mode::Visual => self.handle_visual_mode(input),
            Mode::Replace(bool) => self.handle_replace(input, *bool),
            Mode::Operator(c) => self.handle_operation_pending(input, *c),
        }
    }

    fn handle_traversal(&mut self, input: Input) {
        match input {
            Input {
                key: Key::Char('h'),
                ..
            } => self.textarea.move_cursor(CursorMove::Back),
            Input {
                key: Key::Char('j'),
                ..
            } => self.textarea.move_cursor(CursorMove::Down),
            Input {
                key: Key::Char('k'),
                ..
            } => self.textarea.move_cursor(CursorMove::Up),
            Input {
                key: Key::Char('l'),
                ..
            } => self.textarea.move_cursor(CursorMove::Forward),
            Input {
                key: Key::Char('w' | 'W'),
                ..
            } => self.textarea.move_cursor(CursorMove::WordForward),
            Input {
                key: Key::Char('e' | 'E'),
                ..
            } => self.textarea.move_cursor(CursorMove::WordEnd),
            Input {
                key: Key::Char('b' | 'B'),
                ..
            } => self.textarea.move_cursor(CursorMove::WordBack),
            Input {
                key: Key::Char('^' | '_'),
                ..
            } => self.textarea.move_cursor(CursorMove::Head),
            Input {
                key: Key::Char('$'),
                ..
            } => self.textarea.move_cursor(CursorMove::End),
            Input {
                key: Key::Char('{'),
                ..
            } => self.textarea.move_cursor(CursorMove::ParagraphBack),
            Input {
                key: Key::Char('}'),
                ..
            } => self.textarea.move_cursor(CursorMove::ParagraphForward),
            _ => {}
        }
    }

    fn handle_normal_mode(&mut self, input: Input) -> AppEvent {
        match input {
            Input {
                key: Key::Char('q'),
                ..
            } => AppEvent::Quit,
            Input {
                key: Key::Char('i'),
                ..
            } => AppEvent::InsertMode,
            Input {
                key: Key::Char('I'),
                ..
            } => {
                self.textarea.move_cursor(CursorMove::Head);
                AppEvent::InsertMode
            }
            Input {
                key: Key::Char('a'),
                ..
            } => {
                if Self::is_before_line_end(&self.textarea) {
                    self.textarea.move_cursor(CursorMove::Forward);
                }
                AppEvent::InsertMode
            }
            Input {
                key: Key::Char('A'),
                ..
            } => {
                self.textarea.move_cursor(CursorMove::End);
                AppEvent::InsertMode
            }
            Input {
                key: Key::Char('o'),
                ..
            } => {
                self.textarea.move_cursor(CursorMove::End);
                self.textarea.insert_newline();
                AppEvent::InsertMode
            }
            Input {
                key: Key::Char('O'),
                ..
            } => {
                self.textarea.move_cursor(CursorMove::Head);
                self.textarea.insert_newline();
                self.textarea.move_cursor(CursorMove::Up);
                AppEvent::InsertMode
            }
            Input {
                key: Key::Char('x'),
                ..
            } => {
                self.textarea.delete_next_char();
                AppEvent::NoOp
            }
            Input {
                key: Key::Char('X'),
                ..
            } => {
                self.textarea.delete_char();
                AppEvent::NoOp
            }
            Input {
                key: Key::Char('v'),
                ..
            } => AppEvent::VisualMode,
            Input {
                key: Key::Char('r'),
                ctrl: false,
                ..
            } => AppEvent::ReplaceMode(false),
            Input {
                key: Key::Char('R'),
                ..
            } => AppEvent::ReplaceMode(true),
            Input {
                key: Key::Char(op @ ('y' | 'c' | 'd')),
                ..
            } => AppEvent::OperationMode(op),
            Input {
                key: Key::Char('u' | 'U'),
                ..
            } => {
                self.textarea.undo();
                AppEvent::NoOp
            }
            Input {
                key: Key::Char('r'),
                ctrl: true,
                ..
            } => {
                self.textarea.redo();
                AppEvent::NoOp
            }
            Input {
                key: Key::Char('p'),
                ..
            } => {
                self.textarea.paste();
                AppEvent::NoOp
            }
            Input {
                key: Key::Enter, ..
            } => AppEvent::Submit,
            _ => {
                self.handle_traversal(input);
                AppEvent::NoOp
            }
        }
    }

    fn handle_insert_mode(&mut self, input: Input) -> AppEvent {
        self.textarea.input_without_shortcuts(input);
        AppEvent::NoOp
    }

    fn handle_visual_mode(&mut self, input: Input) -> AppEvent {
        match input {
            Input {
                key: Key::Char('v'),
                ..
            } => {
                self.textarea.cancel_selection();
                AppEvent::NormalMode
            }
            Input {
                key: Key::Char('u' | 'U'),
                ..
            } => AppEvent::NormalMode,
            Input {
                key: Key::Char(op @ ('y' | 'c' | 'd')),
                ..
            } => self.handle_operation(op),
            Input {
                key: Key::Char('p'),
                ..
            } => {
                self.textarea.paste();
                AppEvent::NormalMode
            }
            _ => {
                self.handle_traversal(input);
                AppEvent::NoOp
            }
        }
    }

    fn handle_replace(&mut self, input: Input, replace_mode: bool) -> AppEvent {
        if let Key::Char(c) = input.key {
            // Replace the character under the cursor
            if Self::is_before_line_end(&self.textarea)
                || self.textarea.lines()[self.textarea.cursor().0].len() == self.textarea.cursor().1
            {
                self.textarea.delete_next_char();
                self.textarea.insert_char(c);
            }

            if !replace_mode {
                return AppEvent::NormalMode;
            }
        }

        AppEvent::NoOp
    }

    fn handle_operation_pending(&mut self, input: Input, op: char) -> AppEvent {
        match input {
            Input {
                key: Key::Char('y' | 'c' | 'd'),
                ..
            } => {
                self.textarea.move_cursor(CursorMove::Head);
                self.textarea.start_selection();
                self.textarea.move_cursor(CursorMove::End);
            }
            _ => {
                self.textarea.start_selection();
                self.handle_traversal(input)
            }
        }

        self.handle_operation(op)
    }

    fn handle_operation(&mut self, op: char) -> AppEvent {
        match op {
            'y' => {
                self.textarea.copy();
                AppEvent::NormalMode
            }
            'c' => {
                self.textarea.cut();
                AppEvent::InsertMode
            }
            'd' => {
                self.textarea.cut();
                AppEvent::NormalMode
            }
            _ => AppEvent::NoOp,
        }
    }

    fn is_before_line_end(textarea: &TextArea<'a>) -> bool {
        let cursor = textarea.cursor();
        cursor.1 < textarea.lines()[cursor.0].len().saturating_sub(1)
    }
}
