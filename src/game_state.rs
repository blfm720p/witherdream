use crate::dialogue::{Dialogue, Choice};
use crate::dream_world::DreamWorld;
use crate::inventory::Inventory;
use crate::item::Item;
use crate::maze::Maze;
use crate::npc::NPC;
use crate::player::Player;
use crate::settings::Settings;
use crate::ui::{MenuOption, SettingsMenu, StartMenu};
use ggez::graphics::{Canvas, Color, DrawParam, Image, Mesh, Rect, Sampler, Text};
use ggez::input::keyboard::KeyCode;
use ggez::{event, GameResult};
use image;
use rand::{rng, Rng};
use rodio::{OutputStream, Sink};
use std::time::Instant;
use ffmpeg_next as ffmpeg;

const SCREEN_WIDTH: f32 = 800.0;
const SCREEN_HEIGHT: f32 = 600.0;

#[derive(Clone, Copy, PartialEq)]
#[allow(dead_code)]
pub enum GameMode {
    StartMenu,
    Settings,
    Credits,
    Awake,
    Sleeping,
    Dreaming,
}

#[derive(Clone, Copy, PartialEq)]
pub enum TransitionState {
    None,
    FadingToWake,
    FadingToDream,
}

#[derive(Clone)]
pub struct Particle {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub life: f32,
}

pub struct GameState {
    pub player: Player,
    pub mode: GameMode,
    pub current_world: Option<DreamWorld>,
    pub bed_x: f32,
    pub bed_y: f32,
    pub bicycle: Item,
    pub knife: Item,
    pub npcs: Vec<NPC>,
    pub transition_state: TransitionState,
    pub transition_alpha: f32,
    pub transition_timer: f32,
    pub knife_image: Option<Image>,
    pub bicycle_image: Option<Image>,
    pub bed_image: Option<Image>,
    pub player_image: Option<Image>,
    pub npc1_image: Option<Image>,
    pub npc2_image: Option<Image>,
    pub _title_start_time: Instant,
    pub _stream: OutputStream,
    pub _sink: Sink,
    pub video_frames: Vec<Image>,
    pub current_frame: usize,
    pub last_frame_time: Instant,
    pub dialogue: Option<Dialogue>,
    pub inventory: Inventory,
    pub maze: Option<Maze>,
    pub start_menu: StartMenu,
    pub settings_menu: SettingsMenu,
    pub settings: Settings,
    pub wake_progress: f32,
    pub dust_particles: Vec<Particle>,
    pub bicycle_speed_boost: bool,
}

impl GameState {
    pub fn new(ctx: &mut ggez::Context) -> GameResult<Self> {
        // Load images...
        let knife_image = load_image(ctx, "images/knife_large.png");
        let bicycle_image = load_image(ctx, "images/bicycle.png");
        let bed_image = load_image(ctx, "images/bed.png");
        let player_image = load_image(ctx, "images/player.png");
        let npc1_image = load_image(ctx, "images/npc1.png");
        let npc2_image = load_image(ctx, "images/npc2.png");

        // Audio
        ffmpeg::init().unwrap();
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();

        let video_frames = load_video_frames(ctx, "videos/intro.mp4");

        Ok(Self {
            player: Player::new(),
            mode: GameMode::StartMenu,
            current_world: None,
            bed_x: SCREEN_WIDTH / 2.0 - 30.0,
            bed_y: SCREEN_HEIGHT / 2.0 - 20.0,
            bicycle: Item::new(100.0, 100.0),
            knife: Item::new(200.0, 200.0),
            npcs: vec![
                NPC::new(400.0, 300.0, "Mysterious Figure"),
                NPC::new(600.0, 150.0, "Dream Guardian"),
            ],
            transition_state: TransitionState::None,
            transition_alpha: 0.0,
            transition_timer: 0.0,
            knife_image,
            bicycle_image,
            bed_image,
            player_image,
            npc1_image,
            npc2_image,
            _title_start_time: Instant::now(),
            _stream,
            _sink: sink,
            video_frames,
            current_frame: 0,
            last_frame_time: Instant::now(),
            dialogue: None,
            inventory: Inventory::new(),
            maze: None,
            start_menu: StartMenu::new(),
            settings_menu: SettingsMenu::new(),
            settings: Settings::new(),
            wake_progress: 0.0,
            dust_particles: Vec::new(),
            bicycle_speed_boost: false,
        })
    }

    // Methods for transitions, etc.

    pub fn wake_up(&mut self) {
        self.transition_state = TransitionState::FadingToWake;
        self.transition_alpha = 0.0;
        self.transition_timer = 0.0;
    }

    pub fn go_to_sleep(&mut self) {
        self.mode = GameMode::Sleeping;
        self.current_world = Some(DreamWorld::random());
        self.maze = Some(Maze::new());
    }

    pub fn enter_dream(&mut self) {
        self.transition_state = TransitionState::FadingToDream;
        self.transition_alpha = 0.0;
        self.transition_timer = 0.0;
    }

    pub fn update_transition(&mut self, dt: f32) {
        const TRANSITION_DURATION: f32 = 1.0;

        match self.transition_state {
            TransitionState::FadingToWake => {
                self.transition_timer += dt;
                self.transition_alpha = (self.transition_timer / TRANSITION_DURATION).min(1.0);

                if self.transition_timer >= TRANSITION_DURATION {
                    self.mode = GameMode::Awake;
                    self.current_world = None;
                    self.player = Player::new();
                    self.transition_state = TransitionState::None;
                    self.transition_alpha = 0.0;
                    self.transition_timer = 0.0;
                    self.wake_progress = 0.0;
                }
            }
            TransitionState::FadingToDream => {
                self.transition_timer += dt;
                self.transition_alpha = (self.transition_timer / TRANSITION_DURATION).min(1.0);

                if self.transition_timer >= TRANSITION_DURATION {
                    self.mode = GameMode::Dreaming;
                    self.transition_state = TransitionState::None;
                    self.transition_alpha = 0.0;
                    self.transition_timer = 0.0;
                }
            }
            TransitionState::None => {}
        }
    }

    pub fn update_particles(&mut self, dt: f32) {
        self.dust_particles.retain_mut(|p| {
            p.x += p.vx * dt;
            p.y += p.vy * dt;
            p.life -= dt;
            p.life > 0.0
        });
    }

    pub fn add_dust_particle(&mut self, x: f32, y: f32) {
        let mut rng = rng();
        self.dust_particles.push(Particle {
            x,
            y,
            vx: rng.random_range(-50.0..50.0),
            vy: rng.random_range(-50.0..50.0),
            life: 1.0,
        });
    }
}

impl event::EventHandler<ggez::GameError> for GameState {
    fn update(&mut self, ctx: &mut ggez::Context) -> GameResult<()> {
        let dt = ctx.time.delta().as_secs_f32();

        self.update_transition(dt);
        self.update_particles(dt);

        match self.mode {
            GameMode::StartMenu => {
                self.start_menu.update(ctx);
                if ctx.keyboard.is_key_just_pressed(KeyCode::Return) {
                    match self.start_menu.selected {
                        MenuOption::Start => self.mode = GameMode::Awake,
                        MenuOption::Settings => {
                            self.settings_menu.is_open = true;
                            self.mode = GameMode::Settings;
                        }
                        MenuOption::Credits => self.mode = GameMode::Credits,
                    }
                }
            }
            GameMode::Settings => {
                if ctx.keyboard.is_key_just_pressed(KeyCode::Escape) {
                    self.settings_menu.is_open = false;
                    self.mode = GameMode::StartMenu;
                }
            }
            GameMode::Credits => {
                if ctx.keyboard.is_key_just_pressed(KeyCode::Escape) {
                    self.mode = GameMode::StartMenu;
                }
            }
            GameMode::Awake => {
                if self.transition_state == TransitionState::None {
                    self.player.update(ctx, &self.settings, false);

                    let dx = self.player.x - self.bed_x;
                    let dy = self.player.y - self.bed_y;
                    let distance = (dx * dx + dy * dy).sqrt();

                    if distance < 50.0 && ctx.keyboard.is_key_just_pressed(self.settings.get_key("interact")) {
                        self.dialogue = Some(Dialogue::new("The bed is so comfortable, I want to sleep here forever...".to_string(), "Bed".to_string()).with_choices(vec![
                            Choice { text: "Lay down".to_string(), action: "lay_down".to_string() },
                            Choice { text: "Cancel".to_string(), action: "cancel".to_string() },
                        ]));
                    }

                    if ctx.keyboard.is_key_just_pressed(self.settings.get_key("inventory")) {
                        self.inventory.is_open = !self.inventory.is_open;
                    }
                }
            }
            GameMode::Sleeping => {
                if self.transition_state == TransitionState::None {
                    if ctx.keyboard.is_key_just_pressed(KeyCode::Space) {
                        self.enter_dream();
                    }
                }
            }
            GameMode::Dreaming => {
                if self.transition_state == TransitionState::None {
                    let old_x = self.player.x;
                    let old_y = self.player.y;
                    self.player.update(ctx, &self.settings, self.bicycle_speed_boost);

                    // Maze collision
                    if let Some(ref maze) = self.maze {
                        if maze.is_wall(self.player.x + 40.0, self.player.y + 40.0) {
                            self.player.x = old_x;
                            self.player.y = old_y;
                        }
                    }

                    // Bicycle collection and speed
                    if !self.bicycle.collected {
                        let player_rect = Rect::new(self.player.x, self.player.y, 80.0, 80.0);
                        let bike_rect = Rect::new(self.bicycle.x, self.bicycle.y, 30.0, 20.0);
                        if player_rect.overlaps(&bike_rect) {
                            self.bicycle.collected = true;
                            self.inventory.add_item("Bicycle".to_string());
                            self.bicycle_speed_boost = true;
                        }
                    }

                    if self.bicycle_speed_boost {
                        // Add dust
                        self.add_dust_particle(self.player.x + 40.0, self.player.y + 40.0);
                    }

                    // Knife collection
                    if !self.knife.collected {
                        let player_rect = Rect::new(self.player.x, self.player.y, 80.0, 80.0);
                        let knife_rect = Rect::new(self.knife.x, self.knife.y, 20.0, 20.0);
                        if player_rect.overlaps(&knife_rect) {
                            self.knife.collected = true;
                            self.inventory.add_item("Knife".to_string());
                        }
                    }

                    // NPC interaction
                    for npc in &self.npcs {
                        let dx = self.player.x - npc.x;
                        let dy = self.player.y - npc.y;
                        let distance = (dx * dx + dy * dy).sqrt();
                        if distance < 100.0 && ctx.keyboard.is_key_just_pressed(self.settings.get_key("interact")) {
                            self.dialogue = Some(Dialogue::new(format!("Hello, I am {}", npc.name), npc.name.to_string()));
                        }
                    }

                    // Wake up with hold Z
                    if ctx.keyboard.is_key_pressed(self.settings.get_key("interact")) {
                        self.wake_progress += dt;
                        if self.wake_progress >= 2.0 { // 2 seconds to fill
                            self.wake_up();
                        }
                    } else {
                        self.wake_progress = 0.0;
                    }

                    if ctx.keyboard.is_key_just_pressed(self.settings.get_key("inventory")) {
                        self.inventory.is_open = !self.inventory.is_open;
                    }
                }
            }
        }

        // Handle dialogue
        if let Some(ref mut dialogue) = self.dialogue {
            dialogue.update(ctx);
            if ctx.keyboard.is_key_just_pressed(KeyCode::Return) {
                if let Some(action) = dialogue.select() {
                    match action {
                        "lay_down" => {
                            self.go_to_sleep();
                            self.dialogue = None;
                        }
                        "cancel" => {
                            self.dialogue = None;
                        }
                        _ => {
                            self.dialogue = None;
                        }
                    }
                } else {
                    self.dialogue = None;
                }
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> GameResult<()> {
        let background_color = match self.mode {
            GameMode::StartMenu | GameMode::Settings | GameMode::Credits => Color::BLACK,
            GameMode::Awake => Color::from_rgb(200, 200, 255),
            GameMode::Sleeping => Color::BLACK,
            GameMode::Dreaming => self.current_world.unwrap().background_color,
        };

        let mut canvas = Canvas::from_frame(ctx, background_color);

        match self.mode {
            GameMode::StartMenu => {
                // Video background
                if !self.video_frames.is_empty() {
                    let now = Instant::now();
                    if now.duration_since(self.last_frame_time).as_millis() > 33 {
                        self.current_frame = (self.current_frame + 1) % self.video_frames.len();
                        self.last_frame_time = now;
                    }
                    let frame = &self.video_frames[self.current_frame];
                    canvas.draw(frame, DrawParam::default().scale([
                        SCREEN_WIDTH / frame.width() as f32,
                        SCREEN_HEIGHT / frame.height() as f32
                    ]));
                }

                self.start_menu.draw(&mut canvas, ctx)?;
            }
            GameMode::Settings => {
                self.settings_menu.draw(&mut canvas, ctx)?;
            }
            GameMode::Credits => {
                let text = Text::new("Credits: Made with ggez");
                canvas.draw(&text, DrawParam::default().dest([300.0, 300.0]).color(Color::WHITE));
            }
            GameMode::Awake => {
                self.player.draw(&mut canvas, ctx, self.player_image.as_ref())?;

                if let Some(ref bed_img) = self.bed_image {
                    canvas.set_sampler(Sampler::nearest_clamp());
                    canvas.draw(bed_img, DrawParam::default().dest([self.bed_x, self.bed_y]).scale([2.0, 2.0]));
                } else {
                    let bed_rect = Rect::new(self.bed_x, self.bed_y, 120.0, 80.0);
                    let bed_mesh = Mesh::new_rectangle(ctx, ggez::graphics::DrawMode::fill(), bed_rect, Color::from_rgb(139, 69, 19))?;
                    canvas.draw(&bed_mesh, DrawParam::default());
                }

                let text = Text::new("Press Z near bed to sleep");
                canvas.draw(&text, DrawParam::default().dest([10.0, 10.0]));
            }
            GameMode::Sleeping => {
                let text = Text::new("Sleeping... Press SPACE to dream");
                canvas.draw(&text, DrawParam::default().dest([SCREEN_WIDTH / 2.0 - 150.0, SCREEN_HEIGHT / 2.0]));
            }
            GameMode::Dreaming => {
                // Draw maze
                if let Some(ref maze) = self.maze {
                    maze.draw(&mut canvas, ctx)?;
                }

                self.player.draw(&mut canvas, ctx, self.player_image.as_ref())?;

                // Draw bicycle
                if !self.bicycle.collected {
                    if let Some(ref bike_img) = self.bicycle_image {
                        canvas.set_sampler(Sampler::nearest_clamp());
                        canvas.draw(bike_img, DrawParam::default().dest([self.bicycle.x, self.bicycle.y]).scale([2.0, 2.0]));
                    } else {
                        let bike_rect = Rect::new(self.bicycle.x, self.bicycle.y, 60.0, 40.0);
                        let bike_mesh = Mesh::new_rectangle(ctx, ggez::graphics::DrawMode::fill(), bike_rect, Color::RED)?;
                        canvas.draw(&bike_mesh, DrawParam::default());
                    }
                }

                // Draw knife
                if !self.knife.collected {
                    if let Some(ref knife_img) = self.knife_image {
                        canvas.set_sampler(Sampler::nearest_clamp());
                        canvas.draw(knife_img, DrawParam::default().dest([self.knife.x, self.knife.y]).scale([1.0, 1.0]));
                    } else {
                        let knife_rect = Rect::new(self.knife.x, self.knife.y, 40.0, 40.0);
                        let knife_mesh = Mesh::new_rectangle(ctx, ggez::graphics::DrawMode::fill(), knife_rect, Color::from_rgb(128, 128, 128))?;
                        canvas.draw(&knife_mesh, DrawParam::default());
                    }
                }

                // Draw NPCs
                for (i, npc) in self.npcs.iter().enumerate() {
                    let npc_image = if i == 0 { self.npc1_image.as_ref() } else { self.npc2_image.as_ref() };
                    if let Some(img) = npc_image {
                        canvas.set_sampler(Sampler::nearest_clamp());
                        canvas.draw(img, DrawParam::default().dest([npc.x, npc.y]).scale([100.0 / img.width() as f32, 100.0 / img.height() as f32]));
                    } else {
                        let npc_rect = Rect::new(npc.x, npc.y, 100.0, 100.0);
                        let npc_mesh = Mesh::new_rectangle(ctx, ggez::graphics::DrawMode::fill(), npc_rect, Color::GREEN)?;
                        canvas.draw(&npc_mesh, DrawParam::default());
                    }
                }

                // Draw particles
                for particle in &self.dust_particles {
                    let alpha = particle.life;
                    let particle_rect = Rect::new(particle.x, particle.y, 5.0, 5.0);
                    let particle_mesh = Mesh::new_rectangle(ctx, ggez::graphics::DrawMode::fill(), particle_rect, Color::from_rgba(139, 69, 19, (alpha * 255.0) as u8))?;
                    canvas.draw(&particle_mesh, DrawParam::default());
                }

                // Draw world name
                if let Some(world) = self.current_world {
                    let text = Text::new(format!("Dreaming in: {}", world.name));
                    canvas.draw(&text, DrawParam::default().dest([10.0, 10.0]));
                }

                // Draw wake up bar
                let bar_width = 200.0;
                let bar_height = 20.0;
                let bar_x = 10.0;
                let bar_y = 50.0;
                let fill_width = (self.wake_progress / 2.0) * bar_width;
                let bar_rect = Rect::new(bar_x, bar_y, bar_width, bar_height);
                let bar_mesh = Mesh::new_rectangle(ctx, ggez::graphics::DrawMode::fill(), bar_rect, Color::BLACK)?;
                canvas.draw(&bar_mesh, DrawParam::default());
                let fill_rect = Rect::new(bar_x, bar_y, fill_width, bar_height);
                let fill_mesh = Mesh::new_rectangle(ctx, ggez::graphics::DrawMode::fill(), fill_rect, Color::GREEN)?;
                canvas.draw(&fill_mesh, DrawParam::default());

                let text = Text::new("Hold Z to wake up");
                canvas.draw(&text, DrawParam::default().dest([10.0, 40.0]));
            }
        }

        // Draw dialogue
        if let Some(ref dialogue) = self.dialogue {
            dialogue.draw(&mut canvas, ctx)?;
        }

        // Draw inventory
        self.inventory.draw(&mut canvas, ctx)?;

        // Transition overlay
        if self.transition_state != TransitionState::None {
            let overlay_color = match self.transition_state {
                TransitionState::FadingToWake => Color::from_rgba(255, 255, 255, (self.transition_alpha * 255.0) as u8),
                TransitionState::FadingToDream => Color::from_rgba(255, 255, 255, (self.transition_alpha * 255.0) as u8),
                TransitionState::None => Color::from_rgba(0, 0, 0, 0),
            };

            let overlay_rect = Rect::new(0.0, 0.0, SCREEN_WIDTH, SCREEN_HEIGHT);
            let overlay_mesh = Mesh::new_rectangle(ctx, ggez::graphics::DrawMode::fill(), overlay_rect, overlay_color)?;
            canvas.draw(&overlay_mesh, DrawParam::default());
        }

        canvas.finish(ctx)?;
        Ok(())
    }
}

fn load_video_frames(ctx: &mut ggez::Context, video_path: &str) -> Vec<Image> {
    let mut frames = Vec::new();

    if let Ok(mut ictx) = ffmpeg::format::input(&video_path) {
        let input = ictx
            .streams()
            .best(ffmpeg::media::Type::Video)
            .ok_or(ffmpeg::Error::StreamNotFound)
            .unwrap();

        let video_stream_index = input.index();

        let mut decoder = ffmpeg::codec::context::Context::from_parameters(input.parameters())
            .unwrap()
            .decoder()
            .video()
            .unwrap();

        for (stream, packet) in ictx.packets() {
            if stream.index() == video_stream_index {
                decoder.send_packet(&packet).unwrap();
                let mut decoded = ffmpeg::util::frame::video::Video::empty();
                while decoder.receive_frame(&mut decoded).is_ok() {
                    let mut rgb_frame = ffmpeg::util::frame::video::Video::empty();
                    rgb_frame.set_format(ffmpeg::format::Pixel::RGBA);
                    rgb_frame.set_width(decoded.width());
                    rgb_frame.set_height(decoded.height());

                    let mut scaler = ffmpeg::software::scaling::Context::get(
                        decoded.format(),
                        decoded.width(),
                        decoded.height(),
                        ffmpeg::format::Pixel::RGBA,
                        decoded.width(),
                        decoded.height(),
                        ffmpeg::software::scaling::Flags::BILINEAR,
                    ).unwrap();

                    scaler.run(&decoded, &mut rgb_frame).unwrap();

                    let data = rgb_frame.data(0);
                    let width = rgb_frame.width() as u32;
                    let height = rgb_frame.height() as u32;

                    let image = Image::from_pixels(ctx, data, ggez::graphics::ImageFormat::Rgba8Unorm, width, height);
                    frames.push(image);
                }
            }
        }
    }

    frames
}

fn load_image(ctx: &mut ggez::Context, path: &str) -> Option<Image> {
    match image::open(path) {
        Ok(img) => {
            let mut rgba = img.to_rgba8();
            // Darken the image
            for pixel in rgba.chunks_mut(4) {
                pixel[0] = (pixel[0] as f32 * 0.7) as u8; // R
                pixel[1] = (pixel[1] as f32 * 0.7) as u8; // G
                pixel[2] = (pixel[2] as f32 * 0.7) as u8; // B
                // A remains the same
            }
            let width = rgba.width() as u32;
            let height = rgba.height() as u32;
            let data = rgba.into_raw();
            Some(Image::from_pixels(ctx, &data, ggez::graphics::ImageFormat::Rgba8Unorm, width, height))
        }
        Err(_) => None,
    }
}