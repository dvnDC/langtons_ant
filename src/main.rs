use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color as SdlColor;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

use std::convert::TryInto;

mod engine;

const WINDOW_WIDTH: u32 = 1280;
const WINDOW_HEIGHT: u32 = 720;
const SCALE: u32 = 1;

const GRID_WIDTH: usize = (WINDOW_WIDTH / SCALE) as usize;
const GRID_HEIGHT: usize = (WINDOW_HEIGHT / SCALE) as usize;

const COLOR_GREEN: SdlColor = SdlColor::RGB(0, 255, 0);
const COLOR_BLUE: SdlColor = SdlColor::RGB(0, 0, 255);
const COLOR_TURQUOISE: SdlColor = SdlColor::RGB(64, 224, 208);
const COLOR_DARK_GREEN: SdlColor = SdlColor::RGB(0, 100, 0);
const COLOR_ORANGE: SdlColor = SdlColor::RGB(255, 165, 0);
const COLOR_PURPLE: SdlColor = SdlColor::RGB(128, 0, 128);
const COLOR_YELLOW: SdlColor = SdlColor::RGB(255, 255, 0);
const COLOR_CYAN: SdlColor = SdlColor::RGB(0, 255, 255);
const COLOR_MAGENTA: SdlColor = SdlColor::RGB(255, 0, 255);
const COLOR_GRAY: SdlColor = SdlColor::RGB(128, 128, 128);
const COLOR_LIGHT_BLUE: SdlColor = SdlColor::RGB(173, 216, 230);
const COLOR_DARK_RED: SdlColor = SdlColor::RGB(139, 0, 0);
const COLOR_GOLD: SdlColor = SdlColor::RGB(255, 215, 0);


#[derive(Clone, Copy)]
enum Color {
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

struct Ant {
    x: isize,
    y: isize,
    direction: isize,
}

impl Ant {
    fn new(x: Option<isize>, y: Option<isize>) -> Ant {
        let default_x = x.unwrap_or((GRID_WIDTH / 2) as isize);
        let default_y = y.unwrap_or((GRID_HEIGHT / 2) as isize);

        Ant {
            x: default_x,
            y: default_y,
            direction: 0,
        }
    }

    fn move_forward(&mut self, grid: &mut Vec<Vec<Color>>) {
        match grid[self.y as usize][self.x as usize] {
            Color::White => {
                self.turn_left();
                grid[self.y as usize][self.x as usize] = Color::Black;
            },
            Color::Black => {
                self.turn_left();
                grid[self.y as usize][self.x as usize] = Color::Red;
            },
            Color::Red => {
                self.turn_right();
                grid[self.y as usize][self.x as usize] = Color::Green;
            },
            Color::Green => {
                self.turn_right();
                grid[self.y as usize][self.x as usize] = Color::Blue;
            },
            Color::Blue => {
                self.turn_right();
                grid[self.y as usize][self.x as usize] = Color::Turquoise;
            },
            Color::Turquoise => {
                self.turn_left();
                grid[self.y as usize][self.x as usize] = Color::DarkGreen;
            },
            Color::DarkGreen => {
                self.turn_left();
                grid[self.y as usize][self.x as usize] = Color::Orange;
            },
            Color::Orange => {
                self.turn_left();
                grid[self.y as usize][self.x as usize] = Color::Purple;
            },
            Color::Purple => {
                self.turn_right();
                grid[self.y as usize][self.x as usize] = Color::Yellow;
            },
            Color::Yellow => {
                self.turn_right();
                grid[self.y as usize][self.x as usize] = Color::Cyan;
            },
            Color::Cyan => {
                self.turn_right();
                grid[self.y as usize][self.x as usize] = Color::Magenta;
            },
            Color::Magenta => {
                self.turn_left();
                grid[self.y as usize][self.x as usize] = Color::Gray;
            },
            Color::Gray => {
                self.turn_left();
                grid[self.y as usize][self.x as usize] = Color::LightBlue;
            },
            Color::LightBlue => {
                self.turn_left();
                grid[self.y as usize][self.x as usize] = Color::DarkRed;
            },
            Color::DarkRed => {
                self.turn_left();
                grid[self.y as usize][self.x as usize] = Color::Gold;
            },
            Color::Gold => {
                self.turn_left();
                grid[self.y as usize][self.x as usize] = Color::White;
            },
        }

        self.perform_movement();
    }

    fn turn_right(&mut self) {
        self.direction = (self.direction + 1) % 4;
    }

    fn turn_left(&mut self) {
        self.direction = (self.direction + 3) % 4;
    }

    fn perform_movement(&mut self) {
        match self.direction {
            0 => self.y -= 1, // up
            1 => self.x += 1, // right
            2 => self.y += 1, // down
            3 => self.x -= 1, // left
            _ => panic!("Invalid direction"),
        }
    }

    fn out_of_bounds(&self) -> bool {
        self.x < 0 || self.y < 0 || self.x >= GRID_WIDTH as isize || self.y >= GRID_HEIGHT as isize
    }

    fn teleport(&mut self) {
        self.x = (self.x + GRID_WIDTH as isize) % GRID_WIDTH as isize;
        self.y = (self.y + GRID_HEIGHT as isize) % GRID_HEIGHT as isize;
    }
}

fn main() {
    // engine::menu::Menu::print_menu();
    //todo: delete me later
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Langton's Ant", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas: Canvas<Window> = window.into_canvas().build().unwrap();

    let mut freeze = false;
    let mut speed = 1;

    let mut grid = vec![vec![Color::White; GRID_WIDTH]; GRID_HEIGHT];

    let mut ants = vec![];

    for _ in 0..8 {
        ants.push(Ant::new(Some(GRID_WIDTH as isize / 2), Some(GRID_HEIGHT as isize / 2)));
    }

    let mut step = 0;
    'main_loop: loop {
        for event in sdl_context.event_pump().unwrap().poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'main_loop,
                Event::KeyDown {
                    keycode: Some(Keycode::Space),
                    ..
                } => {
                    freeze = !freeze;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => {
                    speed += 2000;
                    println!("SPEED: {}", speed);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    if speed > 1 {
                        speed -= 2000;
                        println!("SPEED: {}", speed);
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::R),
                    ..
                } => {
                    speed = 1;
                    println!("SPEED: {}", speed);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::T),
                    ..
                } => {
                    speed = 50000;
                    println!("SPEED: {}", speed);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Tab),
                    ..
                } => {
                    println!("ACTUAL STEP: {}", step);
                }
                _ => {}
            }
        }

        if !freeze {
            for _ in 0..speed {
                step += 1;
                for ant in ants.iter_mut() {
                    ant.move_forward(&mut grid);
                    if ant.out_of_bounds() {
                        ant.teleport();
                    }
                }
            }
        }

        canvas.set_draw_color(SdlColor::RGB(0, 0, 0));
        canvas.clear();

        for x in 0..GRID_WIDTH {
            for y in 0..GRID_HEIGHT {
                let rect = Rect::new(
                    ((x as u32 * SCALE)).try_into().unwrap(),
                    ((y as u32 * SCALE)).try_into().unwrap(),
                    SCALE.try_into().unwrap(),
                    SCALE.try_into().unwrap(),
                );
                let color = match grid[y][x] {
                    Color::White => SdlColor::RGB(255, 255, 255),
                    Color::Black => SdlColor::RGB(0, 0, 0),
                    Color::Red => SdlColor::RGB(255, 0, 0),
                    Color::Green => COLOR_GREEN,
                    Color::Blue => COLOR_BLUE,
                    Color::Turquoise => COLOR_TURQUOISE,
                    Color::DarkGreen => COLOR_DARK_GREEN,
                    Color::Orange => COLOR_ORANGE,
                    Color::Purple => COLOR_PURPLE,
                    Color::Yellow => COLOR_YELLOW,
                    Color::Cyan => COLOR_CYAN,
                    Color::Magenta => COLOR_MAGENTA,
                    Color::Gray => COLOR_GRAY,
                    Color::LightBlue => COLOR_LIGHT_BLUE,
                    Color::DarkRed => COLOR_DARK_RED,
                    Color::Gold => COLOR_GOLD,

                };
                canvas.set_draw_color(color);
                canvas.fill_rect(rect).unwrap();
            }
        }

        canvas.present();

        std::thread::sleep(std::time::Duration::from_millis(10));
    }
}
