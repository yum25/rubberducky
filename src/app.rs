use crate::event::{AppEvent, Event, EventHandler};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    DefaultTerminal,
    style::{Color, Modifier, Style},
};
use ratatui_textarea::{CursorMove, Input, Key, TextArea};

use crate::ui::ui;

#[derive(Debug)]
pub enum Mode {
    Normal,
    Insert,
    Replace(bool),
    Visual,
    Operator(char),
}

impl Mode {
    fn cursor_style(&self) -> Style {
        let color = match self {
            Self::Normal => Color::Reset,
            Self::Insert => Color::LightBlue,
            Self::Replace(_) => Color::LightRed,
            Self::Visual => Color::LightYellow,
            Self::Operator(_) => Color::LightGreen,
        };
        Style::default().fg(color).add_modifier(Modifier::REVERSED)
    }
}

/// Application.
#[derive(Debug)]
pub struct App {
    pub running: bool,

    pub mode: Mode,
    pub buffer: String,

    pub textarea: TextArea<'static>,
    pub events: EventHandler,
}

impl Default for App {
    fn default() -> Self {
        let mut textarea = TextArea::default();
        textarea.set_wrap_mode(ratatui_textarea::WrapMode::Word);
        textarea.set_cursor_style(Mode::Normal.cursor_style());
        textarea.set_cursor_line_style(Style::default());

        Self {
            running: true,
            mode: Mode::Normal,
            buffer: String::new(),
            textarea,
            events: EventHandler::new(),
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Run the application's main loop.
    pub async fn run(mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
        while self.running {
            terminal.draw(|frame| ui(frame, &self))?;
            match self.events.next().await? {
                Event::Tick => self.tick(),
                Event::Crossterm(event) => match event {
                    crossterm::event::Event::Key(key_event)
                        if key_event.kind == crossterm::event::KeyEventKind::Press =>
                    {
                        self.handle_key_events(key_event)?
                    }
                    _ => {}
                },
                Event::App(app_event) => match app_event {
                    AppEvent::NormalMode => {
                        self.change_mode(Mode::Normal);
                        self.textarea.cancel_selection();
                    }
                    AppEvent::InsertMode => {
                        self.change_mode(Mode::Insert);
                        self.textarea.cancel_selection();
                    }
                    AppEvent::VisualMode => {
                        self.change_mode(Mode::Visual);
                        self.textarea.start_selection();
                    }
                    AppEvent::Quit => self.quit(),
                },
            }
        }
        Ok(())
    }

    /// Handles the key events and updates the state of [`App`].
    fn handle_key_events(&mut self, key_event: KeyEvent) -> color_eyre::Result<()> {
        match key_event.code {
            KeyCode::Char('c' | 'C') if key_event.modifiers == KeyModifiers::CONTROL => {
                self.events.send(AppEvent::Quit)
            }
            KeyCode::Esc => self.events.send(AppEvent::NormalMode),
            _ => match self.mode {
                Mode::Normal => self.handle_normal_mode(Input::from(key_event)),
                Mode::Insert => self.handle_insert_mode(Input::from(key_event)),
                Mode::Visual => self.handle_visual_mode(Input::from(key_event)),
                Mode::Replace(bool) => self.handle_replace(Input::from(key_event), bool),
                Mode::Operator(c) => self.handle_operation_pending(Input::from(key_event), c),
            },
        }
        Ok(())
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

    fn handle_normal_mode(&mut self, input: Input) {
        match input {
            Input {
                key: Key::Char('q'),
                ..
            } => self.events.send(AppEvent::Quit),
            Input {
                key: Key::Char('i'),
                ..
            } => self.events.send(AppEvent::InsertMode),
            Input {
                key: Key::Char('I'),
                ..
            } => {
                self.events.send(AppEvent::InsertMode);
                self.textarea.move_cursor(CursorMove::Head);
            }
            Input {
                key: Key::Char('a'),
                ..
            } => {
                self.events.send(AppEvent::InsertMode);
                if Self::is_before_line_end(&self.textarea) {
                    self.textarea.move_cursor(CursorMove::Forward);
                }
            }
            Input {
                key: Key::Char('A'),
                ..
            } => {
                self.events.send(AppEvent::InsertMode);
                self.textarea.move_cursor(CursorMove::End);
            }
            Input {
                key: Key::Char('o'),
                ..
            } => {
                self.events.send(AppEvent::InsertMode);
                self.textarea.move_cursor(CursorMove::End);
                self.textarea.insert_newline();
            }
            Input {
                key: Key::Char('O'),
                ..
            } => {
                self.events.send(AppEvent::InsertMode);
                self.textarea.move_cursor(CursorMove::Head);
                self.textarea.insert_newline();
                self.textarea.move_cursor(CursorMove::Up);
            }
            Input {
                key: Key::Char('x'),
                ..
            } => {
                self.textarea.delete_next_char();
            }
            Input {
                key: Key::Char('X'),
                ..
            } => {
                self.textarea.delete_char();
            }
            Input {
                key: Key::Char('v'),
                ..
            } => self.events.send(AppEvent::VisualMode),
            Input {
                key: Key::Char('r'),
                ctrl: false,
                ..
            } => self.change_mode(Mode::Replace(false)),
            Input {
                key: Key::Char('R'),
                ..
            } => self.change_mode(Mode::Replace(true)),
            Input {
                key: Key::Char(op @ ('y' | 'c' | 'd')),
                ..
            } => self.change_mode(Mode::Operator(op)),
            Input {
                key: Key::Char('u' | 'U'),
                ..
            } => {
                self.textarea.undo();
            }
            Input {
                key: Key::Char('r'),
                ctrl: true,
                ..
            } => {
                self.textarea.redo();
            }
            Input {
                key: Key::Char('p'),
                ..
            } => {
                self.textarea.paste();
            }
            _ => self.handle_traversal(input),
        }
    }

    fn handle_insert_mode(&mut self, input: Input) {
        self.textarea.input_without_shortcuts(input);
    }

    fn handle_visual_mode(&mut self, input: Input) {
        match input {
            Input {
                key: Key::Char('v'),
                ..
            } => {
                self.textarea.cancel_selection();
                self.events.send(AppEvent::NormalMode);
            }
            Input {
                key: Key::Char('u' | 'U'),
                ..
            } => self.events.send(AppEvent::NormalMode),
            Input {
                key: Key::Char(op @ ('y' | 'c' | 'd')),
                ..
            } => self.handle_operation(op),
            Input {
                key: Key::Char('p'),
                ..
            } => {
                self.textarea.paste();
                self.events.send(AppEvent::NormalMode);
            }
            _ => self.handle_traversal(input),
        }
    }

    fn handle_replace(&mut self, input: Input, replace_mode: bool) {
        if let Key::Char(c) = input.key {
            // Replace the character under the cursor
            if Self::is_before_line_end(&self.textarea)
                || self.textarea.lines()[self.textarea.cursor().0].len() == self.textarea.cursor().1
            {
                self.textarea.delete_next_char();
                self.textarea.insert_char(c);
            }

            if !replace_mode {
                self.events.send(AppEvent::NormalMode);
            }
        }
    }

    fn handle_operation_pending(&mut self, input: Input, op: char) {
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

        self.handle_operation(op);
    }

    fn handle_operation(&mut self, op: char) {
        match op {
            'y' => {
                self.textarea.copy();
                self.events.send(AppEvent::NormalMode);
            }
            'c' => {
                self.textarea.cut();
                self.events.send(AppEvent::InsertMode);
            }
            'd' => {
                self.textarea.cut();
                self.events.send(AppEvent::NormalMode);
            }
            _ => {}
        }
    }

    fn change_mode(&mut self, mode: Mode) {
        self.mode = mode;
        self.textarea.set_cursor_style(self.mode.cursor_style());
    }

    fn is_before_line_end(textarea: &TextArea<'static>) -> bool {
        let cursor = textarea.cursor();
        cursor.1 < textarea.lines()[cursor.0].len().saturating_sub(1)
    }
    /// Handles the tick event of the terminal.
    ///
    /// The tick event is where you can update the state of your application with any logic that
    /// needs to be updated at a fixed frame rate. E.g. polling a server, updating an animation.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }
}
