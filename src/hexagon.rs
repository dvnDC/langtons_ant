use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::pixels::Color as SdlColor;

use crate::ant::Ant;

pub fn draw_hexagon(
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

pub fn draw_ant(
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