use piston_window::{color, ellipse, rectangle, Context, G2d};
use piston_window::types::Color;

const BLOCK_SIZE:f64 = 25.6;
const RING_COLOR: Color = [0.0, 0.0, 0.5, 1.0]; // Dark blue color
const GLOW_COLOR: Color = [0.0, 0.0, 1.0, 0.5]; // Light blue glow


pub fn to_coord(game_coord: i32) -> f64 {
    (game_coord as f64) * BLOCK_SIZE
}

pub fn to_coord_u32(game_coord: i32) -> u32 {
    to_coord(game_coord) as u32
}
pub fn draw_background(c: &Context, g: &mut G2d) {
    // Draw the glow
    ellipse(
        GLOW_COLOR,
        [50.0, 50.0, 400.0, 400.0], // Adjust these values to fit your window size and position
        c.transform,
        g,
    );

    // Draw the ring
    ellipse(
        RING_COLOR,
        [100.0, 100.0, 300.0, 300.0], // Adjust these values to fit your window size and position
        c.transform,
        g,
    );
}
pub fn draw_block(color: Color, x: i32, y: i32, con: &Context, g: &mut G2d){
    let gui_x = to_coord(x);
    let gui_y = to_coord(y);

    rectangle(
        color,
        [gui_x, gui_y, BLOCK_SIZE, BLOCK_SIZE],
        con.transform,
        g,
    );
}

pub fn draw_rectangle(
    color: Color,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    con: &Context,
    g: &mut G2d,
){
    let x = to_coord(x);
    let y = to_coord(y);

    rectangle(
        color,
        [
            x,
            y,
            BLOCK_SIZE * (width as f64),
            BLOCK_SIZE * (height as f64),
        ],
        con.transform,
        g,
    );
}