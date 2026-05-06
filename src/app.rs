use crate::event::{AppEvent, Event, EventHandler};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::DefaultTerminal;

use crate::ui::ui;

#[derive(Debug)]
pub enum Mode {
    Normal,
    Insert,
    Visual,
}

enum Direction {
    Forwards,
    Backwards,
}

/// Application.
#[derive(Debug)]
pub struct App {
    pub running: bool,

    pub mode: Mode,
    pub cursor: usize,
    pub sequence: String,
    pub buffer: String,

    pub query: String,
    pub events: EventHandler,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            mode: Mode::Normal,
            cursor: 0,
            sequence: String::new(),
            buffer: String::new(),
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
                    AppEvent::NormalMode => self.change_mode(Mode::Normal),
                    AppEvent::InsertMode => self.change_mode(Mode::Insert),
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
                Mode::Normal => self.handle_normal_mode(key_event),
                Mode::Insert => self.handle_insert_mode(key_event),
                Mode::Visual => self.handle_visual_mode(key_event),
            },
        }
        Ok(())
    }

    fn move_cursor(&mut self, delta: usize, direction: Direction) {
        match direction {
            Direction::Forwards => {
                self.cursor = self.cursor.saturating_add(delta).min(self.query.len());
            }
            Direction::Backwards => {
                self.cursor = self.cursor.saturating_sub(delta).min(self.query.len());
            }
        }
    }

    fn handle_normal_mode(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.events.send(AppEvent::Quit),
            KeyCode::Char('i' | 'I') => self.events.send(AppEvent::InsertMode),
            KeyCode::Char('v') => self.events.send(AppEvent::VisualMode),
            KeyCode::Char('d' | 'D') => self.handle_deletion(),
            _ => {}
        }
    }

    fn handle_insert_mode(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char(c) => {
                self.query.insert(self.cursor, c);
                self.move_cursor(1, Direction::Forwards);
            }
            KeyCode::Enter => self.query.push('\n'),
            KeyCode::Backspace => {
                self.query.pop();
                self.move_cursor(1, Direction::Backwards);
            }
            _ => {}
        }
    }

    fn handle_visual_mode(&mut self, key_event: KeyEvent) {}

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
    }
}
