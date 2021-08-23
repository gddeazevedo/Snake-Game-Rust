use piston_window::{ Context, G2d };
use crate::game::constants::FOOD_COLOR;
use crate::game::draw::draw_block;

pub struct Food {
    pub x: i32,
    pub y: i32,
    pub exist: bool,
}

impl Food {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y, exist: true }
    }
}

impl Food {
    pub fn draw(&self, ctx: &Context, g: &mut G2d) {
        if self.exist {
            draw_block(FOOD_COLOR, self.x, self.y, ctx, g);
        }
    }
}
