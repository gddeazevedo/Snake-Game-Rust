#[derive(Debug, Clone)]
pub struct Block {
    pub x: i32,
    pub y: i32,
}

impl Block {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}
