use ggez::graphics::{Canvas, Color, DrawParam, Mesh, Rect, Text};
use ggez::input::keyboard::KeyCode;

#[derive(PartialEq)]
pub enum MenuOption {
    Start,
    Settings,
    Credits,
}

pub struct StartMenu {
    pub selected: MenuOption,
}

impl StartMenu {
    pub fn new() -> Self {
        Self {
            selected: MenuOption::Start,
        }
    }

    pub fn update(&mut self, ctx: &mut ggez::Context) {
        if ctx.keyboard.is_key_just_pressed(KeyCode::Up) {
            self.selected = match self.selected {
                MenuOption::Start => MenuOption::Credits,
                MenuOption::Settings => MenuOption::Start,
                MenuOption::Credits => MenuOption::Settings,
            };
        }
        if ctx.keyboard.is_key_just_pressed(KeyCode::Down) {
            self.selected = match self.selected {
                MenuOption::Start => MenuOption::Settings,
                MenuOption::Settings => MenuOption::Credits,
                MenuOption::Credits => MenuOption::Start,
            };
        }
    }

    pub fn draw(&self, canvas: &mut Canvas, _ctx: &mut ggez::Context) -> ggez::GameResult<()> {
        // Game title
        let title = Text::new("witherdream");
        canvas.draw(&title, DrawParam::default().dest([400.0 - 75.0, 200.0]).color(Color::WHITE));

        // Options
        let start_color = if self.selected == MenuOption::Start { Color::YELLOW } else { Color::WHITE };
        let start_text = Text::new("Start");
        canvas.draw(&start_text, DrawParam::default().dest([400.0 - 25.0, 300.0]).color(start_color));

        let settings_color = if self.selected == MenuOption::Settings { Color::YELLOW } else { Color::WHITE };
        let settings_text = Text::new("Settings");
        canvas.draw(&settings_text, DrawParam::default().dest([400.0 - 35.0, 350.0]).color(settings_color));

        let credits_color = if self.selected == MenuOption::Credits { Color::YELLOW } else { Color::WHITE };
        let credits_text = Text::new("Credits");
        canvas.draw(&credits_text, DrawParam::default().dest([400.0 - 30.0, 400.0]).color(credits_color));

        Ok(())
    }
}

pub struct SettingsMenu {
    pub is_open: bool,
    // For simplicity, just a placeholder
}

impl SettingsMenu {
    pub fn new() -> Self {
        Self { is_open: false }
    }

    pub fn draw(&self, canvas: &mut Canvas, ctx: &mut ggez::Context) -> ggez::GameResult<()> {
        if !self.is_open {
            return Ok(());
        }

        let box_rect = Rect::new(200.0, 150.0, 400.0, 300.0);
        let box_mesh = Mesh::new_rectangle(ctx, ggez::graphics::DrawMode::fill(), box_rect, Color::from_rgba(0, 0, 0, 200))?;
        canvas.draw(&box_mesh, DrawParam::default());

        let text = Text::new("Settings: Keybinds (placeholder)");
        canvas.draw(&text, DrawParam::default().dest([220.0, 170.0]).color(Color::WHITE));

        Ok(())
    }
}