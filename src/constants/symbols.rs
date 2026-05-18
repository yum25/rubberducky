pub mod powerline {
    pub const RIGHT: &str = "\u{E0B0}";
    pub const LEFT: &str = "\u{E0B2}";
    pub const RIGHT_ROUND: &str = "\u{E0B4}";
    pub const LEFT_ROUND: &str = "\u{E0B6}";
}

pub mod caret {
    pub const INPUT: &str = ">";
    pub const INPUT_PENDING: &str = ">";
    pub const OUTPUT: &str = "✓";
    pub const OUTPUT_PENDING: &str = "●";
}

pub mod custom {
    use ratatui::symbols::border::Set;
    pub const TAPERED_BORDER: Set = Set {
        top_left: "▖",
        bottom_left: "▘",
        top_right: " ",
        bottom_right: " ",
        vertical_left: "▌",
        vertical_right: " ",
        horizontal_top: " ",
        horizontal_bottom: " ",
    };
}
