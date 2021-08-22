pub mod constants;
pub mod draw;


use piston_window::*;
use rand::{ thread_rng, Rng };
use draw::draw_rectangle;
use crate::entity::snake::Snake;
use crate::entity::direction::Direction;
use crate::entity::food::Food;
use constants::{
    MOVING_PERIOD,
    RESTART_TIME,
    GAMEOVER_COLOR,
    WIDTH,
    HEIGHT
};


pub struct Game {
    snake: Snake,
    food: Food,
    game_over: bool,
    waiting_time: f64,
}


impl Game {
    pub fn new() -> Self {
        Self {
            snake: Snake::new(2, 2),
            waiting_time: 0.0,
            food: Food::new(WIDTH / 2, HEIGHT / 2),
            game_over: false
        }
    }
}


impl Game {
    pub fn key_pressed(&mut self, key: Key) {
        if self.game_over {
            return;
        }

        let direction = match key {
            Key::Up => Some(Direction::Up),
            Key::Down => Some(Direction::Down),
            Key::Left => Some(Direction::Left),
            Key::Right => Some(Direction::Right),
            _ => None,
        };

        if direction.unwrap() == self.snake.head_direction().opposite() {
            return;
        }

        self.update_snake(direction);
    }

    pub fn draw(&self, ctx: &Context, g: &mut G2d) {
        self.snake.draw(ctx, g);

        self.food.draw(ctx, g);

        if self.game_over {
            draw_rectangle(GAMEOVER_COLOR, 0, 0, WIDTH, HEIGHT, ctx, g);
        }
    }

    pub fn update(&mut self, dt: f64) {
        self.waiting_time += dt;

        if self.game_over {
            if self.waiting_time > RESTART_TIME {
                self.restart();
            }
            return;
        }

        if !self.food.exist {
            self.add_food();
        }

        if self.waiting_time > MOVING_PERIOD {
            self.update_snake(None);
        }
    }

    fn check_eating(&mut self) {
        let (head_x, head_y) = self.snake.head_position();

        if self.food.exist && self.food.x == head_x && self.food.y == head_y {
            self.food.exist = false;
            self.snake.restore_tail();
        }
    }

    fn add_food(&mut self) {
        let mut rng = thread_rng();

        let mut new_food_x = rng.gen_range(0..WIDTH);
        let mut new_food_y = rng.gen_range(0..HEIGHT);

        while self.snake.overlap_tail(new_food_x, new_food_y) {
            new_food_x = rng.gen_range(0..WIDTH);
            new_food_y = rng.gen_range(0..HEIGHT);
        }

        self.food.update(new_food_x, new_food_x, true);
    }

    fn update_snake(&mut self, direction: Option<Direction>) {
        if self.snake.is_alive(direction) {
            self.snake.move_forward(direction);
            self.check_eating();
        } else {
            self.game_over = true;
        }

        self.waiting_time = 0.0;
    }

    fn restart(&mut self) {
        self.snake = Snake::new(2, 2);
        self.waiting_time = 0.0;
        self.food = Food::new(WIDTH / 2, HEIGHT / 2);
        self.game_over = false;
    }
}
