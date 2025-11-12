#[derive(Clone, Copy)]
pub struct NPC {
    pub x: f32,
    pub y: f32,
    pub name: &'static str,
}

impl NPC {
    pub fn new(x: f32, y: f32, name: &'static str) -> Self {
        Self { x, y, name }
    }
}