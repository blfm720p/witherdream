mod player;
mod item;
mod npc;
mod dream_world;
mod dialogue;
mod inventory;
mod maze;
mod ui;
mod settings;
mod game_state;

use ggez::{event, ContextBuilder, GameResult};
use game_state::GameState;

const SCREEN_WIDTH: f32 = 800.0;
const SCREEN_HEIGHT: f32 = 600.0;

fn main() -> GameResult<()> {
    let cb = ContextBuilder::new("witherdream", "author")
        .add_resource_path("./")
        .window_setup(ggez::conf::WindowSetup::default().title("witherdream"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_WIDTH, SCREEN_HEIGHT));

    let (mut ctx, event_loop) = cb.build()?;
    let state = GameState::new(&mut ctx)?;

    event::run(ctx, event_loop, state);
}
