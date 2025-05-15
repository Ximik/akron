pub const FONT: iced::Font = iced::Font::with_name("icons");
pub enum Icon {
    Settings,
    Copy,
    ArrowBigUpDash,
    FolderDown,
    Wallet,
    ArrowBigDownDash,
    ArrowsUpFromLine,
    Store,
    ChevronLeft,
    UserRoundPen,
    Bitcoin,
    Bolt,
    AtSign,
}
impl Icon {
    pub fn as_char(&self) -> char {
        match self {
            Icon::Settings => '\u{E000}',
            Icon::Copy => '\u{E001}',
            Icon::ArrowBigUpDash => '\u{E002}',
            Icon::FolderDown => '\u{E004}',
            Icon::Wallet => '\u{E005}',
            Icon::ArrowBigDownDash => '\u{E006}',
            Icon::ArrowsUpFromLine => '\u{E007}',
            Icon::Store => '\u{E008}',
            Icon::ChevronLeft => '\u{E009}',
            Icon::UserRoundPen => '\u{E00A}',
            Icon::Bitcoin => '\u{E00B}',
            Icon::Bolt => '\u{E00C}',
            Icon::AtSign => '\u{E00D}',
        }
    }
}
