/*
    Default settings
    Resolution: 1500x800
    pub const SCALE: u32 = 10;
    pub const HEXAGONS_HORIZONTAL: u32 = 100;
    pub const HEXAGONS_VERTICAL: u32 = 80;

    Close-up view settings
    Resolution: 1665x740
    pub const SCALE: u32 = 37;
    pub const HEXAGONS_HORIZONTAL: u32 = 30;
    pub const HEXAGONS_VERTICAL: u32 = 25;
 */

pub const SCALE: u32 = 37;
pub const HEXAGONS_HORIZONTAL: u32 = 30;
pub const HEXAGONS_VERTICAL: u32 = 20;

pub const WINDOW_WIDTH: u32 = HEXAGONS_HORIZONTAL * SCALE * 3 / 2;
pub const WINDOW_HEIGHT: u32 = HEXAGONS_VERTICAL * SCALE;

pub const GRID_WIDTH: usize = HEXAGONS_HORIZONTAL as usize;
pub const GRID_HEIGHT: usize = HEXAGONS_VERTICAL as usize;