use piston_window as pw;
use pw::types::Color;
use rand::{ thread_rng, Rng };
use crate::draw::{ draw_block, draw_rectangle };
use crate::snake::{ Direction, Snake };


const FOOD_COLOR: Color = [0.8, 0.0, 0.0, 1.0];
const BORDER_COLOR: Color = [0.0, 0.0, 0.0, 1.0];
const GAMEOVER_COLOR: Color = [0.9, 0.0, 0.0, 0.5];
const MOVING_PERIOD: f64 = 0.1;
const RESTART_TIME: f64 = 1.0;


pub struct Game {
    snake: Snake,

    food_exists: bool,
    food_x: i32,
    food_y: i32,

    width: i32,
    height: i32,

    game_over: bool,
    waiting_time: f64,
}


impl Game {
    pub fn new(width: i32, height: i32) -> Self {
        Self {
            snake: Snake::new(2, 2),
            waiting_time: 0.0,
            food_exists: true,
            food_x: 6,
            food_y: 4,
            width,
            height,
            game_over: false
        }
    }
}
