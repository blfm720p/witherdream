#[derive(Clone, Copy)]
pub struct Item {
    pub x: f32,
    pub y: f32,
    pub collected: bool,
}

impl Item {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y, collected: false }
    }
}