use piston_window::types::Color;

pub const BG_COLOR: Color = [0.5, 0.5, 0.5, 1.0];
pub const FOOD_COLOR: Color = [0.8, 0.0, 0.0, 1.0];
pub const GAMEOVER_COLOR: Color = [0.9, 0.0, 0.0, 0.5];
pub const SNAKE_COLOR: Color = [0.0, 0.8, 0.0, 1.0];


pub const MOVING_PERIOD: f64 = 0.1;
pub const RESTART_TIME: f64 = 1.0;
pub const SCALE: f64 = 25.0; // blocks will be scaled up 25 pixels

pub const WIDTH: i32 = 24;
pub const HEIGHT: i32 = 24;
