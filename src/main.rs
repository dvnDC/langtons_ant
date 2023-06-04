use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color as SdlColor;
use sdl2::rect::Rect;
use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::f64;


use std::convert::TryInto;

mod engine;

const WINDOW_WIDTH: u32 = 1280;
const WINDOW_HEIGHT: u32 = 720;
const SCALE: u32 = 10;

const GRID_WIDTH: usize = (WINDOW_WIDTH / SCALE) as usize;
const GRID_HEIGHT: usize = (WINDOW_HEIGHT / SCALE) as usize;

const COLOR_WHITE: SdlColor = SdlColor::RGB(255, 255, 255);
const COLOR_BLACK: SdlColor = SdlColor::RGB(0, 0, 0);
const COLOR_RED: SdlColor = SdlColor::RGB(255, 0, 0);
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
enum Rotation {
    N, // No change
    R1, // 60° clockwise
    R2, // 120° clockwise
    U, // 180°
    L2, // 120° counter-clockwise
    L1, // 60° counter-clockwise
}

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
    hex_direction: isize,
}

impl Ant {
    fn new(x: Option<isize>, y: Option<isize>) -> Ant {
        let default_x = x.unwrap_or((GRID_WIDTH / 2) as isize);
        let default_y = y.unwrap_or((GRID_HEIGHT / 2) as isize);

        Ant {
            x: default_x,
            y: default_y,
            hex_direction: 0,
        }
    }

    fn move_forward(&mut self, grid: &mut Vec<Vec<Color>>) {
        let current_color = grid[self.y as usize][self.x as usize];
        let (next_color, rotation) = match current_color {
            Color::Black => (Color::Red, Rotation::L1),
            Color::Red => (Color::Turquoise, Rotation::L2),
            Color::Turquoise => (Color::Blue, Rotation::N),
            Color::Blue => (Color::DarkGreen, Rotation::U),
            Color::DarkGreen => (Color::Cyan, Rotation::L2),
            Color::Cyan => (Color::LightBlue, Rotation::L1),
            Color::LightBlue => (Color::Black, Rotation::R2),
            _ => (Color::Black, Rotation::N), // Default case, if needed
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

    fn out_of_bounds(&self) -> bool {
        self.x < 0 || self.y < 0 || self.x >= GRID_WIDTH as isize || self.y >= GRID_HEIGHT as isize
    }

    fn teleport(&mut self) {
        self.x = (self.x + GRID_WIDTH as isize) % GRID_WIDTH as isize;
        self.y = (self.y + GRID_HEIGHT as isize) % GRID_HEIGHT as isize;
    }
}

// Euclidean distance formula
// distance = sqrt((x2 - x1)^2 + (y2 - y1)^2)
fn distance_between_points(a: Point, b: Point) -> f64 {
    let dx = (a.x() - b.x()).pow(2) as f64;
    let dy = (a.y() - b.y()).pow(2) as f64;
    (dx + dy).sqrt()
}

fn intersect_points(y: i32, points: &[Point; 7]) -> (Point, Point) {
    let mut intersections = Vec::new();
    for i in 0..6 {
        let p1 = points[i];
        let p2 = points[i + 1];
        if p1.y() <= y && p2.y() > y || p1.y() > y && p2.y() <= y {
            let x = p1.x() + (y - p1.y()) * (p2.x() - p1.x()) / (p2.y() - p1.y());
            intersections.push(Point::new(x, y));
        }
    }
    (intersections[0], intersections[1])
}

fn draw_hexagon(
    canvas: &mut Canvas<Window>,
    x: i32,
    y: i32,
    size: u32,
    border_color: SdlColor,
    fill_color: SdlColor,
) {
    let points = [
        Point::new(x, y + size as i32 / 2),
        Point::new(x + size as i32 / 2, y),
        Point::new(x + size as i32 * 3 / 2, y),
        Point::new(x + size as i32 * 2, y + size as i32 / 2),
        Point::new(x + size as i32 * 3 / 2, y + size as i32),
        Point::new(x + size as i32 / 2, y + size as i32),
        Point::new(x, y + size as i32 / 2),
    ];

    canvas.set_draw_color(border_color);
    canvas.draw_lines(points.as_ref()).unwrap();

    canvas.set_draw_color(fill_color);
    for i in y..y + size as i32 {
        let (start, end) = intersect_points(i, &points);
        canvas.draw_line(start, end).unwrap();
    }
}

fn draw_ant(
    canvas: &mut Canvas<Window>,
    ant: &Ant,
    size: u32,
    color: SdlColor,
) {
    let x = ant.x as u32;
    let y = ant.y as u32;
    let x_pos = (x * size * 3 / 2).try_into().unwrap();
    let y_pos = (y * size + (x % 2) * size / 2).try_into().unwrap();

    draw_hexagon(canvas, x_pos, y_pos, size, color, color);
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

    let mut freeze = true;
    let mut speed = 1;

    let mut grid = vec![vec![Color::Black; GRID_WIDTH]; GRID_HEIGHT];

    let mut ants = vec![];

    ants.push(Ant::new(Some(GRID_WIDTH as isize / 2), Some(GRID_HEIGHT as isize / 2)));

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
                    speed += 1;
                    println!("SPEED: {}", speed);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    if speed > 1 {
                        speed -= 1;
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
                    speed = 50;
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
                let x_pos = (x as u32 * SCALE * 3 / 2).try_into().unwrap();
                let y_pos = (y as u32 * SCALE + (x as u32 % 2) * SCALE / 2).try_into().unwrap();

                let fill_color = match grid[y][x] {
                    Color::Black => COLOR_BLACK,
                    Color::Red => COLOR_RED,
                    Color::Turquoise => COLOR_TURQUOISE,
                    Color::Blue => COLOR_BLUE,
                    Color::DarkGreen => COLOR_DARK_GREEN,
                    Color::Cyan => COLOR_CYAN,
                    Color::LightBlue => COLOR_LIGHT_BLUE,
                    _ => SdlColor::RGB(255, 255, 255),
                };

                let border_color = SdlColor::RGB(0, 0, 0); // Set border color to black

                draw_hexagon(&mut canvas, x_pos, y_pos, SCALE, border_color, fill_color);
            }
        }
        for ant in &ants {
            draw_ant(&mut canvas, ant, SCALE, COLOR_YELLOW); // Use a distinct color for the ant, e.g., yellow
        }

        canvas.present();

        std::thread::sleep(std::time::Duration::from_millis(10));
    }
}
