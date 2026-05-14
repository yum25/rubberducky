use crate::event::{AppEvent, Event, EventHandler};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    DefaultTerminal,
    style::{Color, Modifier, Style},
};
use ratatui_textarea::Input;

use crate::{components::message::Message, ui::ui};

#[derive(Debug, Clone, Copy)]
pub enum Mode {
    Normal,
    Insert,
    Replace(bool),
    Visual,
    Operator(char),
}

impl Mode {
    pub fn cursor_style(&self) -> Style {
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
pub struct App<'a> {
    pub running: bool,

    pub mode: Mode,
    pub user_input: Message<'a>,
    pub messages: Vec<Message<'a>>,
    pub events: EventHandler,
}

impl<'a> Default for App<'a> {
    fn default() -> Self {
        Self {
            running: true,
            mode: Mode::Normal,
            user_input: Message::new(),
            messages: Vec::new(),
            events: EventHandler::new(),
        }
    }
}

impl<'a> App<'a> {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Run the application's main loop.
    pub async fn run(mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
        while self.running {
            terminal.draw(|frame| ui(frame, &mut self))?;
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
                        self.user_input.cancel_selection();
                    }
                    AppEvent::InsertMode => {
                        self.change_mode(Mode::Insert);
                        self.user_input.cancel_selection();
                    }
                    AppEvent::VisualMode => {
                        self.change_mode(Mode::Visual);
                        self.user_input.start_selection();
                    }
                    AppEvent::ReplaceMode(bool) => {
                        self.change_mode(Mode::Replace(bool));
                        self.user_input.cancel_selection();
                    }
                    AppEvent::OperationMode(char) => {
                        self.change_mode(Mode::Operator(char));
                    }
                    AppEvent::Submit => {
                        self.user_input.clear_cursor_style();

                        self.messages.push(self.user_input);
                        self.user_input = Message::new();
                        self.change_mode(Mode::Normal);
                    }
                    AppEvent::Quit => self.quit(),
                    AppEvent::NoOp => {}
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
            _ => {
                let event = self
                    .user_input
                    .handle_key_event(&self.mode, Input::from(key_event));

                self.events.send(event);
            }
        }
        Ok(())
    }
    fn change_mode(&mut self, mode: Mode) {
        self.mode = mode;
        self.user_input.set_cursor_style(self.mode.cursor_style());
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
