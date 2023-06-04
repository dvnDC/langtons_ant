/*
    Default settings:
    SCALE: 10
    HEXAGONS_HORIZONTAL: 100
    HEXAGONS_VERTICAL: 80
 */

pub const SCALE: u32 = 10;
pub const HEXAGONS_HORIZONTAL: u32 = 100;
pub const HEXAGONS_VERTICAL: u32 = 80;

pub const WINDOW_WIDTH: u32 = HEXAGONS_HORIZONTAL * SCALE * 3 / 2;
pub const WINDOW_HEIGHT: u32 = HEXAGONS_VERTICAL * SCALE;

pub const GRID_WIDTH: usize = HEXAGONS_HORIZONTAL as usize;
pub const GRID_HEIGHT: usize = HEXAGONS_VERTICAL as usize;