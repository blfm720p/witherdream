use ggez::input::keyboard::KeyCode;
use std::collections::HashMap;

pub struct Settings {
    pub keybinds: HashMap<String, KeyCode>,
}

impl Settings {
    pub fn new() -> Self {
        let mut keybinds = HashMap::new();
        keybinds.insert("up".to_string(), KeyCode::W);
        keybinds.insert("down".to_string(), KeyCode::S);
        keybinds.insert("left".to_string(), KeyCode::A);
        keybinds.insert("right".to_string(), KeyCode::D);
        keybinds.insert("interact".to_string(), KeyCode::Z);
        keybinds.insert("inventory".to_string(), KeyCode::I);

        Self { keybinds }
    }

    pub fn get_key(&self, action: &str) -> KeyCode {
        *self.keybinds.get(action).unwrap_or(&KeyCode::F1)
    }

}