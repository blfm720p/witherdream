use ggez::graphics::{Canvas, Color, DrawParam, Mesh, Rect, Text};

pub struct Inventory {
    pub items: Vec<String>,
    pub is_open: bool,
}

impl Inventory {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            is_open: false,
        }
    }

    pub fn add_item(&mut self, item: String) {
        self.items.push(item);
    }

    pub fn draw(&self, canvas: &mut Canvas, ctx: &mut ggez::Context) -> ggez::GameResult<()> {
        if !self.is_open {
            return Ok(());
        }

        // Inventory box
        let box_rect = Rect::new(100.0, 100.0, 600.0, 400.0);
        let box_mesh = Mesh::new_rectangle(ctx, ggez::graphics::DrawMode::fill(), box_rect, Color::from_rgba(0, 0, 0, 200))?;
        canvas.draw(&box_mesh, DrawParam::default());

        // Title
        let title = Text::new("INVENTORY");
        canvas.draw(&title, DrawParam::default().dest([320.0, 120.0]).color(Color::WHITE));

        // Items
        for (i, item) in self.items.iter().enumerate() {
            let y = 160.0 + (i as f32 * 30.0);
            let item_text = Text::new(item);
            canvas.draw(&item_text, DrawParam::default().dest([120.0, y]).color(Color::WHITE));
        }

        Ok(())
    }
}