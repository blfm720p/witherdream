use ggez::graphics::Color;
use rand::{rng, Rng};

#[derive(Clone, Copy)]
pub struct DreamWorld {
    pub background_color: Color,
    pub name: &'static str,
}

#[allow(dead_code)]
impl DreamWorld {
    pub fn random() -> Self {
        let worlds = [
            DreamWorld { background_color: Color::from_rgb(100, 50, 150), name: "Purple Forest" },
            DreamWorld { background_color: Color::from_rgb(200, 100, 50), name: "Orange Desert" },
            DreamWorld { background_color: Color::from_rgb(50, 150, 200), name: "Blue Ocean" },
            DreamWorld { background_color: Color::from_rgb(150, 50, 100), name: "Pink Mountains" },
            DreamWorld { background_color: Color::from_rgb(100, 200, 50), name: "Green Fields" },
            DreamWorld { background_color: Color::from_rgb(200, 150, 100), name: "Golden Plains" },
        ];
        let mut rng = rng();
        worlds[rng.random_range(0..worlds.len())]
    }
}