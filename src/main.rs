mod game;
mod entity;

use game::Game;
use game::draw::scale_up_u32;
use game::constants::{ BG_COLOR, WIDTH, HEIGHT };
use piston_window::*;



fn main() {
    let mut window: PistonWindow = WindowSettings::new(
        "Snake",
        [scale_up_u32(WIDTH), scale_up_u32(HEIGHT)],
    ).exit_on_esc(true)
        .build()
        .unwrap();

    let mut game = Game::new();

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
