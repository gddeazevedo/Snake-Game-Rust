use std::collections::LinkedList;
use piston_window::{ Context, G2d };
use crate::game::draw::draw_block;
use crate::entity::direction::Direction;
use crate::entity::block::Block;
use crate::game::constants::{ SNAKE_COLOR, WIDTH, HEIGHT };



pub struct Snake {
    direction: Direction,
    body: LinkedList<Block>,
    tail: Option<Block>,
}


impl Snake {
    pub fn new(x: i32, y: i32) -> Self {
        let mut body: LinkedList<Block> = LinkedList::new();

        body.push_back(Block::new(x + 2, y));
        body.push_back(Block::new(x + 1, y));
        body.push_back(Block::new(x, y));

        Self {
            direction: Direction::Right,
            body,
            tail: None,
        }
    }
}


impl Snake {
    pub fn draw(&self, ctx: &Context, g: &mut G2d) {
        for block in &self.body {
            draw_block(SNAKE_COLOR, block.x, block.y, ctx, g);
        }
    }

    pub fn head_position(&self) -> (i32, i32) {
        // unwrap takes the value out of Some(v) if it exists
        let head_block = self.body.front().unwrap();
        (head_block.x, head_block.y)
    }

    pub fn move_forward(&mut self, direction: Option<Direction>) {
        match direction {
            Some(dir) => self.direction = dir,
            None => ()
        }

        let (last_x, last_y) = self.head_position();

        let new_block = match self.direction {
            Direction::Up => Block::new(last_x, last_y - 1),
            Direction::Down => Block::new(last_x, last_y + 1),
            Direction::Left => Block::new(last_x - 1, last_y),
            Direction::Right => Block::new(last_x + 1, last_y),
        };

        self.body.push_front(new_block);
        let removed_block = self.body.pop_back().unwrap();
        self.tail = Some(removed_block);
    }

    pub fn head_direction(&self) -> Direction {
        self.direction
    }

    pub fn next_head(&self, direction: Option<Direction>) -> (i32, i32) {
        let (head_x, head_y) = self.head_position();
        let mut current_direction = self.direction;

        match direction {
            Some(new_direction) => current_direction = new_direction,
            None => (),
        }

        match current_direction {
            Direction::Up => (head_x, head_y - 1),
            Direction::Down => (head_x, head_y + 1),
            Direction::Left => (head_x - 1, head_y),
            Direction::Right => (head_x + 1, head_y),
        }
    }

    pub fn restore_tail(&mut self) {
        let tail = self.tail.clone().unwrap();
        self.body.push_back(tail);
    }

    pub fn is_alive(&self, direction: Option<Direction>) -> bool {
        let (next_x, next_y) = self.next_head(direction);

        if self.overlap_tail(next_x, next_y) {
            return false;
        }

        next_x > -1 && next_y > -1 && next_x < WIDTH && next_y < HEIGHT
    }

    pub fn overlap_tail(&self, x: i32, y: i32) -> bool {
        let mut counter = 0;

        for body_part in &self.body {
            if x == body_part.x && y == body_part.y {
                return true;
            }

            counter += 1;

            if counter == self.body.len() - 1 {
                break;
            }
        }

        false
    }
}
