use ggez::graphics::{Canvas, Color, DrawParam, Mesh, Rect, Text};
use ggez::input::keyboard::KeyCode;

#[derive(Clone)]
pub struct Choice {
    pub text: String,
    pub action: String,
}

pub struct Dialogue {
    pub text: String,
    pub speaker: String,
    pub is_active: bool,
    pub choices: Vec<Choice>,
    pub selected_choice: usize,
}

impl Dialogue {
    pub fn new(text: String, speaker: String) -> Self {
        Self {
            text,
            speaker,
            is_active: true,
            choices: Vec::new(),
            selected_choice: 0,
        }
    }

    pub fn with_choices(mut self, choices: Vec<Choice>) -> Self {
        self.choices = choices;
        self
    }

    pub fn update(&mut self, ctx: &mut ggez::Context) {
        if !self.is_active {
            return;
        }

        if ctx.keyboard.is_key_just_pressed(KeyCode::Up) && self.selected_choice > 0 {
            self.selected_choice -= 1;
        }
        if ctx.keyboard.is_key_just_pressed(KeyCode::Down) && self.selected_choice < self.choices.len() - 1 {
            self.selected_choice += 1;
        }
    }

    pub fn select(&self) -> Option<&str> {
        if self.choices.is_empty() {
            None
        } else {
            Some(&self.choices[self.selected_choice].action)
        }
    }

    pub fn draw(&self, canvas: &mut Canvas, ctx: &mut ggez::Context) -> ggez::GameResult<()> {
        if !self.is_active {
            return Ok(());
        }

        // Dialogue box
        let box_rect = Rect::new(50.0, 400.0, 700.0, 150.0);
        let box_mesh = Mesh::new_rectangle(ctx, ggez::graphics::DrawMode::fill(), box_rect, Color::from_rgba(0, 0, 0, 200))?;
        canvas.draw(&box_mesh, DrawParam::default());

        // Speaker name
        let speaker_text = Text::new(format!("{}:", self.speaker));
        canvas.draw(&speaker_text, DrawParam::default().dest([60.0, 410.0]).color(Color::WHITE));

        // Dialogue text
        let dialogue_text = Text::new(&self.text);
        canvas.draw(&dialogue_text, DrawParam::default().dest([60.0, 440.0]).color(Color::WHITE));

        // Choices
        if !self.choices.is_empty() {
            for (i, choice) in self.choices.iter().enumerate() {
                let y = 470.0 + (i as f32 * 20.0);
                let color = if i == self.selected_choice { Color::YELLOW } else { Color::WHITE };
                let choice_text = Text::new(&choice.text);
                canvas.draw(&choice_text, DrawParam::default().dest([60.0, y]).color(color));
            }
        }

        Ok(())
    }
}