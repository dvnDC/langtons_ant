use crate::color::Color;
use crate::rotation::Rotation;
use crate::config::*;

pub struct Ant {
    pub x: isize,
    pub y: isize,
    hex_direction: isize,
}

impl Ant {
    pub fn new(x: Option<isize>, y: Option<isize>) -> Ant {
        let default_x = x.unwrap_or((GRID_WIDTH / 2) as isize);
        let default_y = y.unwrap_or((GRID_HEIGHT / 2) as isize);

        Ant {
            x: default_x,
            y: default_y,
            hex_direction: 0,
        }
    }

    pub fn move_forward(&mut self, grid: &mut Vec<Vec<Color>>) {
        let current_color = grid[self.y as usize][self.x as usize];
        let (next_color, rotation) = match current_color {
            Color::Black => (Color::Red, Rotation::L1),
            Color::Red => (Color::Turquoise, Rotation::L2),
            Color::Turquoise => (Color::Blue, Rotation::N),
            Color::Blue => (Color::DarkGreen, Rotation::U),
            Color::DarkGreen => (Color::Cyan, Rotation::L2),
            Color::Cyan => (Color::LightBlue, Rotation::L1),
            Color::LightBlue => (Color::Black, Rotation::R2),
            _ => (Color::Black, Rotation::N),
        };

        self.rotate(rotation);
        grid[self.y as usize][self.x as usize] = next_color;
        self.perform_hex_movement();

        println!("Ant position: ({}, {}), hex_direction: {}", self.x, self.y, self.hex_direction);
    }


    fn perform_hex_movement(&mut self) {
        let is_even_col = self.x % 2 == 0;
        match self.hex_direction {
            0 => {
                self.x += 1;
            }
            1 => {
                if is_even_col {
                    self.y += 1;
                } else {
                    self.x += 1;
                    self.y += 1;
                }
            }
            2 => {
                if is_even_col {
                    self.y -= 1;
                } else {
                    self.x += 1;
                    self.y -= 1;
                }
            }
            3 => {
                self.x -= 1;
            }
            4 => {
                if is_even_col {
                    self.y -= 1;
                } else {
                    self.x -= 1;
                    self.y -= 1;
                }
            }
            5 => {
                if is_even_col {
                    self.y += 1;
                } else {
                    self.x -= 1;
                    self.y += 1;
                }
            }
            _ => panic!("Invalid hex_direction"),
        }
    }


    fn rotate(&mut self, rotation: Rotation) {
        match rotation {
            Rotation::N => {}
            Rotation::R1 => {
                self.hex_direction = (self.hex_direction + 1) % 6;
            }
            Rotation::R2 => {
                self.hex_direction = (self.hex_direction + 2) % 6;
            }
            Rotation::U => {
                self.hex_direction = (self.hex_direction + 3) % 6;
            }
            Rotation::L2 => {
                self.hex_direction = (self.hex_direction + 4) % 6;
            }
            Rotation::L1 => {
                self.hex_direction = (self.hex_direction + 5) % 6;
            }
        }
    }

    pub fn out_of_bounds(&self) -> bool {
        self.x < 0 || self.y < 0 || self.x >= GRID_WIDTH as isize || self.y >= GRID_HEIGHT as isize
    }

    pub fn teleport(&mut self) {
        self.x = (self.x + GRID_WIDTH as isize) % GRID_WIDTH as isize;
        self.y = (self.y + GRID_HEIGHT as isize) % GRID_HEIGHT as isize;
    }
}