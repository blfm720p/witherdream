use crate::settings::Settings;
use ggez::graphics::{Canvas, Color, DrawParam, Image, Mesh, Rect, Sampler};

const PLAYER_SIZE: f32 = 80.0;
const PLAYER_SPEED: f32 = 150.0;

pub struct Player {
    pub x: f32,
    pub y: f32,
}

impl Player {
    pub fn new() -> Self {
        Self {
            x: 800.0 / 2.0 - PLAYER_SIZE / 2.0,
            y: 600.0 / 2.0 - PLAYER_SIZE / 2.0,
        }
    }

    pub fn update(&mut self, ctx: &mut ggez::Context, settings: &Settings, speed_boost: bool) {
        let dt = ctx.time.delta().as_secs_f32();
        let speed = if speed_boost { PLAYER_SPEED * 1.5 } else { PLAYER_SPEED };

        if ctx.keyboard.is_key_pressed(settings.get_key("up")) {
            self.y -= speed * dt;
        }
        if ctx.keyboard.is_key_pressed(settings.get_key("down")) {
            self.y += speed * dt;
        }
        if ctx.keyboard.is_key_pressed(settings.get_key("left")) {
            self.x -= speed * dt;
        }
        if ctx.keyboard.is_key_pressed(settings.get_key("right")) {
            self.x += speed * dt;
        }

        // Keep player within screen bounds
        self.x = self.x.max(0.0).min(800.0 - PLAYER_SIZE);
        self.y = self.y.max(0.0).min(600.0 - PLAYER_SIZE);
    }

    pub fn draw(&self, canvas: &mut Canvas, ctx: &mut ggez::Context, player_image: Option<&Image>) -> ggez::GameResult<()> {
        if let Some(img) = player_image {
            canvas.set_sampler(Sampler::nearest_clamp());
            canvas.draw(img, DrawParam::default().dest([self.x, self.y]).scale([PLAYER_SIZE / img.width() as f32, PLAYER_SIZE / img.height() as f32]));
        } else {
            let rect = Rect::new(self.x, self.y, PLAYER_SIZE, PLAYER_SIZE);
            let mesh = Mesh::new_rectangle(ctx, ggez::graphics::DrawMode::fill(), rect, Color::WHITE)?;
            canvas.draw(&mesh, DrawParam::default());
        }
        Ok(())
    }
}