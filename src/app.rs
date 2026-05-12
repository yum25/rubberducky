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
    pub cursor: usize,
    pub sequence: String,
    pub buffer: String,

    pub textarea: TextArea<'static>,
    pub query: String,
    pub events: EventHandler,
}

impl Default for App {
    fn default() -> Self {
        let mut textarea = TextArea::default();
        textarea.set_wrap_mode(ratatui_textarea::WrapMode::Word);
        textarea.set_cursor_style(Mode::Normal.cursor_style());

        Self {
            running: true,
            mode: Mode::Normal,
            cursor: 0,
            sequence: String::new(),
            buffer: String::new(),
            textarea,
            query: String::new(),
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
                    }
                    AppEvent::InsertMode => {
                        self.change_mode(Mode::Insert);
                    }
                    AppEvent::VisualMode => self.change_mode(Mode::Visual),
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
                _ => {}
            },
        }
        Ok(())
    }

    fn handle_normal_mode(&mut self, input: Input) {
        match input {
            Input {
                key: Key::Char('q'),
                ..
            } => self.events.send(AppEvent::Quit),
            Input {
                key: Key::Char('i' | 'I'),
                ..
            } => self.events.send(AppEvent::InsertMode),
            Input {
                key: Key::Char('v'),
                ..
            } => self.events.send(AppEvent::VisualMode),
            // Handle directional inputs
            Input {
                key: Key::Char('d' | 'D'),
                ..
            } => self.handle_deletion(),
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
            _ => {}
        }
    }

    fn handle_insert_mode(&mut self, input: Input) {
        self.textarea.input_without_shortcuts(input);
    }

    fn handle_visual_mode(&mut self, input: Input) {}

    fn handle_deletion(&mut self) {}

    /// Handles the tick event of the terminal.
    ///
    /// The tick event is where you can update the state of your application with any logic that
    /// needs to be updated at a fixed frame rate. E.g. polling a server, updating an animation.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn change_mode(&mut self, mode: Mode) {
        self.mode = mode;
        self.textarea.set_cursor_style(self.mode.cursor_style());
    }
}
