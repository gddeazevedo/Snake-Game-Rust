use piston_window::{ types::Color, rectangle, Context, G2d };

const BLOCK_SIZE: f64 = 25.0; // blocks will be scaled up 25 pixels


pub fn to_coord(coord: i32) -> f64 {
    (coord as f64) * BLOCK_SIZE
}


pub fn to_coord_u32(coord: i32) -> u32 {
    to_coord(coord) as u32
}


pub fn draw_block(color: Color, x: i32, y: i32, ctx: &Context, g: &mut G2d) {
    let gui_x = to_coord(x);
    let gui_y = to_coord(y);
    let rect = [gui_x, gui_y, BLOCK_SIZE, BLOCK_SIZE];

    rectangle(color, rect, ctx.transform, g);
}


pub fn draw_rectangle(color: Color, x: i32, y: i32, width: i32, height: i32, ctx: &Context, g: &mut G2d) {
    let gui_x = to_coord(x);
    let gui_y = to_coord(y);
    let rect = [
        gui_x,
        gui_y,
        BLOCK_SIZE * (width as f64),
        BLOCK_SIZE * (height as f64)
    ];

    rectangle(color, rect, ctx.transform, g);
}
