mod draw;
mod snake;
mod game;

use game::Game;
use draw::to_coord_u32;
use piston_window::*;
use piston_window::types::Color;


const BG_COLOR: Color = [0.5, 0.5, 0.5, 1.0];


fn main() {
    let (width, height) = (25, 25);
    let mut window: PistonWindow = WindowSettings::new(
        "Snake",
        [to_coord_u32(width), to_coord_u32(height)],
    ).exit_on_esc(true)
        .build()
        .unwrap();

    let mut game = Game::new(width, height);

    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            game.key_pressed(key);
        }

        window.draw_2d(&event, |ctx, g, _| {
            clear(BG_COLOR, g);
            game.draw(&ctx, g);
        });

        event.update(|arg| {
            game.update(arg.dt);
        });
    }
}
