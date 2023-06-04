use sdl2::pixels::Color as SdlColor;

pub const COLOR_WHITE: SdlColor = SdlColor::RGB(255, 255, 255);
pub const COLOR_BLACK: SdlColor = SdlColor::RGB(0, 0, 0);
pub const COLOR_RED: SdlColor = SdlColor::RGB(255, 0, 0);
pub const COLOR_GREEN: SdlColor = SdlColor::RGB(0, 255, 0);
pub const COLOR_BLUE: SdlColor = SdlColor::RGB(0, 0, 255);
pub const COLOR_TURQUOISE: SdlColor = SdlColor::RGB(64, 224, 208);
pub const COLOR_DARK_GREEN: SdlColor = SdlColor::RGB(0, 100, 0);
pub const COLOR_ORANGE: SdlColor = SdlColor::RGB(255, 165, 0);
pub const COLOR_PURPLE: SdlColor = SdlColor::RGB(128, 0, 128);
pub const COLOR_YELLOW: SdlColor = SdlColor::RGB(255, 255, 0);
pub const COLOR_CYAN: SdlColor = SdlColor::RGB(0, 255, 255);
pub const COLOR_MAGENTA: SdlColor = SdlColor::RGB(255, 0, 255);
pub const COLOR_GRAY: SdlColor = SdlColor::RGB(128, 128, 128);
pub const COLOR_LIGHT_BLUE: SdlColor = SdlColor::RGB(173, 216, 230);
pub const COLOR_DARK_RED: SdlColor = SdlColor::RGB(139, 0, 0);
pub const COLOR_GOLD: SdlColor = SdlColor::RGB(255, 215, 0);

#[derive(Clone, Copy)]
pub enum Color {
    White,
    Black,
    Red,
    Green,
    Blue,
    Turquoise,
    DarkGreen,
    Orange,
    Purple,
    Yellow,
    Cyan,
    Magenta,
    Gray,
    LightBlue,
    DarkRed,
    Gold
}