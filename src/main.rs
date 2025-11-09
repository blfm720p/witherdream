use ggez::{event, ContextBuilder, GameResult};
use ggez::graphics::{self, Canvas, Color, DrawParam, Image, Mesh, Rect, Text};
use ggez::input::keyboard::KeyCode;
use rand::{rng, Rng};
use std::time::Instant;
use rodio::{OutputStream, Sink};
use ffmpeg_next as ffmpeg;

const SCREEN_WIDTH: f32 = 800.0;
const SCREEN_HEIGHT: f32 = 600.0;
const PLAYER_SIZE: f32 = 80.0;
const PLAYER_SPEED: f32 = 200.0;

#[derive(Clone, Copy)]
struct DreamWorld {
    background_color: Color,
    name: &'static str,
}

impl DreamWorld {
    fn random() -> Self {
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

#[derive(Clone, Copy)]
struct Item {
    x: f32,
    y: f32,
    collected: bool,
}

impl Item {
    fn new(x: f32, y: f32) -> Self {
        Self { x, y, collected: false }
    }
}

#[derive(Clone, Copy)]
struct NPC {
    x: f32,
    y: f32,
    name: &'static str,
}

impl NPC {
    fn new(x: f32, y: f32, name: &'static str) -> Self {
        Self { x, y, name }
    }
}

struct Player {
    x: f32,
    y: f32,
}

impl Player {
    fn new() -> Self {
        Self {
            x: SCREEN_WIDTH / 2.0 - PLAYER_SIZE / 2.0,
            y: SCREEN_HEIGHT / 2.0 - PLAYER_SIZE / 2.0,
        }
    }

    fn update(&mut self, ctx: &mut ggez::Context) {
        let dt = ctx.time.delta().as_secs_f32();

        if ctx.keyboard.is_key_pressed(KeyCode::W) || ctx.keyboard.is_key_pressed(KeyCode::Up) {
            self.y -= PLAYER_SPEED * dt;
        }
        if ctx.keyboard.is_key_pressed(KeyCode::S) || ctx.keyboard.is_key_pressed(KeyCode::Down) {
            self.y += PLAYER_SPEED * dt;
        }
        if ctx.keyboard.is_key_pressed(KeyCode::A) || ctx.keyboard.is_key_pressed(KeyCode::Left) {
            self.x -= PLAYER_SPEED * dt;
        }
        if ctx.keyboard.is_key_pressed(KeyCode::D) || ctx.keyboard.is_key_pressed(KeyCode::Right) {
            self.x += PLAYER_SPEED * dt;
        }

        // Keep player within screen bounds
        self.x = self.x.max(0.0).min(SCREEN_WIDTH - PLAYER_SIZE);
        self.y = self.y.max(0.0).min(SCREEN_HEIGHT - PLAYER_SIZE);
    }

    fn draw(&self, canvas: &mut Canvas, ctx: &mut ggez::Context, player_image: Option<&Image>) -> GameResult<()> {
        if let Some(img) = player_image {
            canvas.draw(img, DrawParam::default().dest([self.x, self.y]).scale([PLAYER_SIZE / img.width() as f32, PLAYER_SIZE / img.height() as f32]));
        } else {
            let rect = Rect::new(self.x, self.y, PLAYER_SIZE, PLAYER_SIZE);
            let mesh = Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), rect, Color::WHITE)?;
            canvas.draw(&mesh, DrawParam::default());
        }
        Ok(())
    }
}

#[derive(PartialEq)]
enum GameMode {
    TitleScreen,
    Awake,
    Sleeping,
    Dreaming,
}

#[derive(PartialEq)]
enum TransitionState {
    None,
    FadingToSleep,
    FadingToWake,
    FadingToDream,
}

struct GameState {
    player: Player,
    mode: GameMode,
    current_world: Option<DreamWorld>,
    bed_x: f32,
    bed_y: f32,
    bicycle: Item,
    knife: Item,
    npcs: Vec<NPC>,
    transition_state: TransitionState,
    transition_alpha: f32,
    transition_timer: f32,
    knife_image: Option<Image>,
    bicycle_image: Option<Image>,
    bed_image: Option<Image>,
    player_image: Option<Image>,
    npc1_image: Option<Image>,
    npc2_image: Option<Image>,
    title_start_time: Instant,
    _stream: OutputStream,
    sink: Sink,
    video_frames: Vec<Image>,
    current_frame: usize,
    last_frame_time: Instant,
}

impl GameState {
    fn new(ctx: &mut ggez::Context) -> GameResult<Self> {
        let knife_image = match Image::from_path(ctx, "/images/knife_large.png") {
            Ok(img) => {
                println!("Knife image loaded successfully");
                Some(img)
            }
            Err(e) => {
                println!("Failed to load knife image: {:?}", e);
                None
            }
        };
        let bicycle_image = match Image::from_path(ctx, "/images/bicycle.png") {
            Ok(img) => {
                println!("Bicycle image loaded successfully");
                Some(img)
            }
            Err(e) => {
                println!("Failed to load bicycle image: {:?}", e);
                None
            }
        };
        let bed_image = match Image::from_path(ctx, "/images/bed.png") {
            Ok(img) => {
                println!("Bed image loaded successfully");
                Some(img)
            }
            Err(e) => {
                println!("Failed to load bed image: {:?}", e);
                None
            }
        };
        let player_image = match Image::from_path(ctx, "/images/player.png") {
            Ok(img) => {
                println!("Player image loaded successfully");
                Some(img)
            }
            Err(e) => {
                println!("Failed to load player image: {:?}", e);
                None
            }
        };
        let npc1_image = match Image::from_path(ctx, "/images/npc1.png") {
            Ok(img) => {
                println!("NPC1 image loaded successfully");
                Some(img)
            }
            Err(e) => {
                println!("Failed to load NPC1 image: {:?}", e);
                None
            }
        };
        let npc2_image = match Image::from_path(ctx, "/images/npc2.png") {
            Ok(img) => {
                println!("NPC2 image loaded successfully");
                Some(img)
            }
            Err(e) => {
                println!("Failed to load NPC2 image: {:?}", e);
                None
            }
        };

        // Initialize audio
        ffmpeg::init().unwrap();
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();

        // Load video frames
        let video_frames = load_video_frames(ctx, "videos/intro.mp4");

        Ok(Self {
            player: Player::new(),
            mode: GameMode::TitleScreen,
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
            title_start_time: Instant::now(),
            _stream,
            sink,
            video_frames,
            current_frame: 0,
            last_frame_time: Instant::now(),
        })
    }

    fn go_to_sleep(&mut self) {
        self.transition_state = TransitionState::FadingToSleep;
        self.transition_alpha = 0.0;
        self.transition_timer = 0.0;
        self.current_world = Some(DreamWorld::random());
    }

    fn wake_up(&mut self) {
        self.transition_state = TransitionState::FadingToWake;
        self.transition_alpha = 0.0;
        self.transition_timer = 0.0;
    }

    fn enter_dream(&mut self) {
        self.transition_state = TransitionState::FadingToDream;
        self.transition_alpha = 0.0;
        self.transition_timer = 0.0;
    }

    fn update_transition(&mut self, dt: f32) {
        const TRANSITION_DURATION: f32 = 1.0; // 1 second transition

        match self.transition_state {
            TransitionState::FadingToSleep => {
                self.transition_timer += dt;
                self.transition_alpha = (self.transition_timer / TRANSITION_DURATION).min(1.0);

                if self.transition_timer >= TRANSITION_DURATION {
                    self.mode = GameMode::Sleeping;
                    self.transition_state = TransitionState::None;
                    self.transition_alpha = 0.0;
                    self.transition_timer = 0.0;
                }
            }
            TransitionState::FadingToWake => {
                self.transition_timer += dt;
                self.transition_alpha = (self.transition_timer / TRANSITION_DURATION).min(1.0);

                if self.transition_timer >= TRANSITION_DURATION {
                    self.mode = GameMode::Awake;
                    self.current_world = None;
                    self.player = Player::new(); // Reset player position
                    self.transition_state = TransitionState::None;
                    self.transition_alpha = 0.0;
                    self.transition_timer = 0.0;
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
}

impl event::EventHandler<ggez::GameError> for GameState {
    fn update(&mut self, ctx: &mut ggez::Context) -> GameResult<()> {
        let dt = ctx.time.delta().as_secs_f32();

        // Update transitions
        self.update_transition(dt);

        match self.mode {
            GameMode::TitleScreen => {
                if ctx.keyboard.is_key_just_pressed(KeyCode::Space) {
                    self.mode = GameMode::Awake;
                    // Stop any playing audio when starting the game
                    self.sink.stop();
                }
            }
            GameMode::Awake => {
                if self.transition_state == TransitionState::None {
                    self.player.update(ctx);

                    // Check if player is near bed
                    let dx = self.player.x - self.bed_x;
                    let dy = self.player.y - self.bed_y;
                    let distance = (dx * dx + dy * dy).sqrt();

                    if distance < 50.0 && ctx.keyboard.is_key_just_pressed(KeyCode::Z) {
                        self.go_to_sleep();
                    }
                }
            }
            GameMode::Sleeping => {
                if self.transition_state == TransitionState::None {
                    // Automatically enter dream after a short delay
                    if ctx.keyboard.is_key_just_pressed(KeyCode::Space) {
                        self.enter_dream();
                    }
                }
            }
            GameMode::Dreaming => {
                if self.transition_state == TransitionState::None {
                    self.player.update(ctx);

                    // Check for item collection
                    let player_rect = Rect::new(self.player.x, self.player.y, PLAYER_SIZE, PLAYER_SIZE);

                    if !self.bicycle.collected {
                        let bike_rect = Rect::new(self.bicycle.x, self.bicycle.y, 30.0, 20.0);
                        if player_rect.overlaps(&bike_rect) {
                            self.bicycle.collected = true;
                        }
                    }

                    if !self.knife.collected {
                        let knife_rect = Rect::new(self.knife.x, self.knife.y, 20.0, 20.0);
                        if player_rect.overlaps(&knife_rect) {
                            self.knife.collected = true;
                        }
                    }

                    // Press Z to wake up
                    if ctx.keyboard.is_key_pressed(KeyCode::Z) {
                        self.wake_up();
                    }
                }
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> GameResult<()> {
        let background_color = match self.mode {
            GameMode::TitleScreen => Color::BLACK,
            GameMode::Awake => Color::from_rgb(200, 200, 255), // Light blue for real world
            GameMode::Sleeping => Color::BLACK,
            GameMode::Dreaming => self.current_world.unwrap().background_color,
        };

        let mut canvas = Canvas::from_frame(ctx, background_color);

        match self.mode {
            GameMode::TitleScreen => {
                // Update video frame
                if !self.video_frames.is_empty() {
                    let now = Instant::now();
                    if now.duration_since(self.last_frame_time).as_millis() > 33 { // ~30 FPS
                        self.current_frame = (self.current_frame + 1) % self.video_frames.len();
                        self.last_frame_time = now;
                    }

                    // Draw current video frame as background
                    let frame = &self.video_frames[self.current_frame];
                    canvas.draw(frame, DrawParam::default().scale([
                        SCREEN_WIDTH / frame.width() as f32,
                        SCREEN_HEIGHT / frame.height() as f32
                    ]));
                }

                // Draw title
                let title_text = Text::new("witherdream");
                canvas.draw(&title_text, DrawParam::default()
                    .dest([SCREEN_WIDTH / 2.0 - 150.0, SCREEN_HEIGHT / 2.0 - 50.0])
                    .color(Color::WHITE));

                // Draw instructions
                let start_text = Text::new("Press SPACE to start");
                canvas.draw(&start_text, DrawParam::default()
                    .dest([SCREEN_WIDTH / 2.0 - 100.0, SCREEN_HEIGHT / 2.0 + 20.0])
                    .color(Color::WHITE));
            }
            GameMode::Awake => {
                self.player.draw(&mut canvas, ctx, self.player_image.as_ref())?;

                // Draw bed
                if let Some(ref bed_img) = self.bed_image {
                    canvas.draw(bed_img, DrawParam::default().dest([self.bed_x, self.bed_y]).scale([1.0, 1.0]));
                } else {
                    let bed_rect = Rect::new(self.bed_x, self.bed_y, 60.0, 40.0);
                    let bed_mesh = Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), bed_rect, Color::from_rgb(139, 69, 19))?;
                    canvas.draw(&bed_mesh, DrawParam::default());
                }

                // Draw instructions
                let text = Text::new("Press Z near bed to sleep");
                canvas.draw(&text, DrawParam::default().dest([10.0, 10.0]));
            }
            GameMode::Sleeping => {
                let text = Text::new("Sleeping... Press SPACE to dream");
                canvas.draw(&text, DrawParam::default().dest([SCREEN_WIDTH / 2.0 - 150.0, SCREEN_HEIGHT / 2.0]));
            }
            GameMode::Dreaming => {
                self.player.draw(&mut canvas, ctx, self.player_image.as_ref())?;

                // Draw bicycle if not collected
                if !self.bicycle.collected {
                    if let Some(ref bike_img) = self.bicycle_image {
                        canvas.draw(bike_img, DrawParam::default().dest([self.bicycle.x, self.bicycle.y]).scale([1.0, 1.0]));
                    } else {
                        let bike_rect = Rect::new(self.bicycle.x, self.bicycle.y, 30.0, 20.0);
                        let bike_mesh = Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), bike_rect, Color::from_rgb(255, 0, 0))?;
                        canvas.draw(&bike_mesh, DrawParam::default());
                    }
                }

                // Draw knife if not collected
                if !self.knife.collected {
                    if let Some(ref knife_img) = self.knife_image {
                        canvas.draw(knife_img, DrawParam::default().dest([self.knife.x, self.knife.y]).scale([1.0, 1.0]));
                    } else {
                        let knife_rect = Rect::new(self.knife.x, self.knife.y, 40.0, 40.0);
                        let knife_mesh = Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), knife_rect, Color::from_rgb(128, 128, 128))?;
                        canvas.draw(&knife_mesh, DrawParam::default());
                    }
                }

                // Draw NPCs
                for (i, npc) in self.npcs.iter().enumerate() {
                    let npc_image = if i == 0 { self.npc1_image.as_ref() } else { self.npc2_image.as_ref() };
                    if let Some(img) = npc_image {
                        canvas.draw(img, DrawParam::default().dest([npc.x, npc.y]).scale([100.0 / img.width() as f32, 100.0 / img.height() as f32]));
                    } else {
                        let npc_rect = Rect::new(npc.x, npc.y, 100.0, 100.0);
                        let npc_mesh = Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), npc_rect, Color::from_rgb(0, 255, 0))?;
                        canvas.draw(&npc_mesh, DrawParam::default());
                    }
                }

                // Draw world name
                if let Some(world) = self.current_world {
                    let text = Text::new(format!("Dreaming in: {}", world.name));
                    canvas.draw(&text, DrawParam::default().dest([10.0, 10.0]));
                }

                // Draw wake up instructions
                let text = Text::new("Hold Z to wake up");
                canvas.draw(&text, DrawParam::default().dest([10.0, 40.0]));

                // Draw collected items
                let mut y_pos = 70.0;
                if self.bicycle.collected {
                    let text = Text::new("Bicycle collected!");
                    canvas.draw(&text, DrawParam::default().dest([10.0, y_pos]));
                    y_pos += 20.0;
                }
                if self.knife.collected {
                    let text = Text::new("Knife collected!");
                    canvas.draw(&text, DrawParam::default().dest([10.0, y_pos]));
                }
            }
        }

        // Draw transition overlay
        if self.transition_state != TransitionState::None {
            let overlay_color = match self.transition_state {
                TransitionState::FadingToSleep => Color::from_rgba(0, 0, 0, (self.transition_alpha * 255.0) as u8),
                TransitionState::FadingToWake => Color::from_rgba(255, 255, 255, (self.transition_alpha * 255.0) as u8),
                TransitionState::FadingToDream => Color::from_rgba(255, 255, 255, (self.transition_alpha * 255.0) as u8),
                TransitionState::None => Color::from_rgba(0, 0, 0, 0),
            };

            let overlay_rect = Rect::new(0.0, 0.0, SCREEN_WIDTH, SCREEN_HEIGHT);
            let overlay_mesh = Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), overlay_rect, overlay_color)?;
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
                    // Convert frame to RGBA
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

                    // Create image from frame data
                    let data = rgb_frame.data(0);
                    let width = rgb_frame.width() as u32;
                    let height = rgb_frame.height() as u32;

                    // Convert to ggez Image
                    let image = Image::from_pixels(ctx, data, ggez::graphics::ImageFormat::Rgba8Unorm, width, height);
                    frames.push(image);
                }
            }
        }
    }

    frames
}

fn main() -> GameResult<()> {
    let cb = ContextBuilder::new("witherdream", "author")
        .window_setup(ggez::conf::WindowSetup::default().title("witherdream"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_WIDTH, SCREEN_HEIGHT));

    let (mut ctx, event_loop) = cb.build()?;
    let state = GameState::new(&mut ctx)?;

    event::run(ctx, event_loop, state);
}
