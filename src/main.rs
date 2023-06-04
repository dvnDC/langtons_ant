/* Langton's Ant program created by Damian Cichosz */

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Point;
use sdl2::render::Canvas;
use std::f64;
use std::convert::TryInto;
use sdl2::video::Window;

mod engine;
mod ant;
mod color;
mod hexagon;
mod rotation;
mod config;

use ant::Ant;
use color::*;
use hexagon::{draw_ant, draw_hexagon};
use rotation::Rotation;
use crate::engine::menu;
use crate::config::*;


fn main() {
    // menu::Menu::print_menu(); //todo: fix me before using
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
    let mut sleep_duration = 100;

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
                    sleep_duration += 10;
                    println!("Delay between moves: {} ms", sleep_duration);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    if sleep_duration > 10 {
                        sleep_duration -= 10;
                        println!("Delay between moves: {} ms", sleep_duration);
                    }
                    else if sleep_duration == 10 {
                        sleep_duration = 1;
                        println!("Delay between moves: {} ms", sleep_duration);
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::R),
                    ..
                } => {
                    speed = 1;
                    sleep_duration = 100;
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

        canvas.set_draw_color(COLOR_BLACK);
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
                    _ => COLOR_WHITE,
                };

                let border_color = COLOR_BLACK;

                draw_hexagon(&mut canvas, x_pos, y_pos, SCALE, border_color, fill_color);
            }
        }
        for ant in &ants {
            draw_ant(&mut canvas, ant, SCALE, COLOR_YELLOW);
        }

        canvas.present();

        std::thread::sleep(std::time::Duration::from_millis(sleep_duration));
    }
}
