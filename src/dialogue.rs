use ggez::graphics::{Canvas, Color, DrawParam, Mesh, Rect, Text};

pub struct Dialogue {
    pub text: String,
    pub speaker: String,
    pub is_active: bool,
}

impl Dialogue {
    pub fn new(text: String, speaker: String) -> Self {
        Self {
            text,
            speaker,
            is_active: true,
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

        Ok(())
    }
}