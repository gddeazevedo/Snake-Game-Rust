use piston_window::{ types::Color, rectangle, Context, G2d };
use crate::constants::SCALE;


fn scale_up(coord: i32) -> f64 {
    (coord as f64) * SCALE
}


pub fn scale_up_u32(value: i32) -> u32 {
    scale_up(value) as u32
}


pub fn draw_block(color: Color, x: i32, y: i32, ctx: &Context, g: &mut G2d) {
    let gui_x = scale_up(x);
    let gui_y = scale_up(y);
    let rect = [gui_x, gui_y, SCALE, SCALE];

    rectangle(color, rect, ctx.transform, g);
}


pub fn draw_rectangle(color: Color, x: i32, y: i32, width: i32, height: i32, ctx: &Context, g: &mut G2d) {
    let gui_x = scale_up(x);
    let gui_y = scale_up(y);
    let rect: [f64; 4] = [
        gui_x,
        gui_y,
        scale_up(width),
        scale_up(height),
    ];

    rectangle(color, rect, ctx.transform, g);
}
