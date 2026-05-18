use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget};

enum ModelPreset {
    StalwartMallard,
    RubberDucky,
    SillyGoose,
}

pub struct StatusCard {
    model: String,
}

impl Widget for StatusCard {
    fn render(self, area: Rect, buf: &mut Buffer) {}
}
