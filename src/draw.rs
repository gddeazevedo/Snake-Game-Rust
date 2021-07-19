use piston_window::{ types::Color, rectangle, Context, G2d };

const BLOCK_SIZE: f64 = 25.0; // blocks will be scaled up 25 pixels


pub fn to_coord(x: i32, y: i32) -> (f64, f64) {
    (
        (x as f64) * BLOCK_SIZE,
        (y as f64) * BLOCK_SIZE,
    )
}


pub fn draw_block(color: Color, x: i32, y: i32, ctx: &Context, g: &mut G2d) {
    let (gui_x, gui_y) = to_coord(x, y);
    let rect = [gui_x, gui_y, BLOCK_SIZE, BLOCK_SIZE];

    rectangle(color, rect, ctx.transform, g);
}


pub fn draw_rectangle(
    color: Color,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    ctx: &Context,
    g: &mut G2d
) {
    let (gui_x, gui_y) = to_coord(x, y);
    let rect = [
        gui_x,
        gui_y,
        BLOCK_SIZE * (width as f64),
        BLOCK_SIZE * (height as f64)
    ];

    rectangle(color, rect, ctx.transform, g);
}
