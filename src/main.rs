#![warn(clippy::unwrap_used)]
#![allow(clippy::uninlined_format_args)]

use ggez::*;
use ggez::conf::{Conf, WindowMode, WindowSetup};
use ggez::event::ScanCode;
use ggez::graphics::{Color, DrawParam, Quad, Rect, Text};
use ggez::input::keyboard::KeyInput;
use ggez::mint::Point2;

enum Direction {
    Left,
    Right,
}

impl Direction {
    /// Returns a [`Direction`] from a [`ScanCode`]
    fn from_scancode(scancode: ScanCode) -> Option<Self> {
        match scancode {
            // ScanCode for [`KeyCode::A`] | [`KeyCode::Left`]
            30 | 57419 => Some(Direction::Left),
            // ScanCode for [`KeyCode::D`] | [`KeyCode::Right`]
            32 | 57421 => Some(Direction::Right),
            _ => None
        }
    }
}

struct PlayerPaddle {
    rect: Rect,
    speed: f32,
    direction: Option<Direction>,
}

impl PlayerPaddle {
    /// Moves the [`PlayerPaddle`] depending on its [`Direction`]
    fn move_self(&mut self) {
        match self.direction {
            Some(Direction::Left) => self.rect.x -= self.speed,
            Some(Direction::Right) => self.rect.x += self.speed,
            None => (),
        }
        self.direction = None;
    }
}

struct Ball {
    position: Point2<f32>,
    radius: f32,
    speed: Point2<f32>,
}

struct WinLoseRect {
    rect: Rect,
    radius: f32,
}

struct GameState {
    delta_time: std::time::Duration,
    has_won: bool,
    win_lose_rect: WinLoseRect,
    player: PlayerPaddle,
    ball: Ball,
}

impl event::EventHandler<GameError> for GameState {
    fn update(&mut self, context: &mut Context) -> Result<(), GameError> {
        // only run at 60 UPS (Updates Per Second)
        while context.time.check_update_time(TARGET_UPS) {
            self.delta_time = context.time.delta();

            // move player
            self.player.move_self();

            // move ball //TODO: TEMP
            self.ball.position.x += self.ball.speed.x;
            self.ball.position.y -= self.ball.speed.y;

            force_player_boundaries(&mut self.player, context);
            ball_wall_collisions(&mut self.ball, context);
            
            if check_game_lose(&self.ball, context) {
                self.has_won = true;
            }
        }

        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> Result<(), GameError> {
        let mut canvas = graphics::Canvas::from_frame(context, Color::from([0.1, 0.5, 0.3, 1.0]));

        // draw fps counter
        let fps = context.time.fps();
        let fps_display = Text::new(format!("FPS: {:.2}", fps));
        canvas.draw(
            &fps_display,
            DrawParam::from([0.0, 0.0]).color(Color::WHITE),
        );

        // draw paddle
        let paddle_rect = &self.player.rect;
        canvas.draw(
            &Quad,
            DrawParam::new()
                .dest(paddle_rect.point())
                .scale(paddle_rect.size())
                .color(Color::WHITE),
        );

        // draw ball
        let ball_mesh_builder = &mut graphics::MeshBuilder::new();
        ball_mesh_builder.circle(
            graphics::DrawMode::fill(),
            self.ball.position,
            self.ball.radius,
            0.69,
            Color::WHITE,
        )?;
        let ball_mesh = graphics::Mesh::from_data(context, ball_mesh_builder.build());
        canvas.draw(&ball_mesh, DrawParam::new());

        // display win or lose
        if self.has_won {
            let mesh_builder = &mut graphics::MeshBuilder::new();
            mesh_builder.rounded_rectangle(
                graphics::DrawMode::fill(),
                self.win_lose_rect.rect,
                self.win_lose_rect.radius,
                Color::WHITE,
            )?;
            let win_lose_mesh = graphics::Mesh::from_data(context, mesh_builder.build());
            canvas.draw(&win_lose_mesh, DrawParam::new());
        } else {
            //TODO: lose
        }
        
        canvas.finish(context)?;
        Ok(())
    }

    fn key_down_event(&mut self, _context: &mut Context, input: KeyInput, _repeated: bool) -> Result<(), GameError> {
        self.player.direction = Direction::from_scancode(input.scancode);

        Ok(())
    }
}

/// Wall collisions of the [`Ball`]
fn ball_wall_collisions(ball: &mut Ball, context: &Context) {
    let window_width = context.gfx.window().inner_size().width as f32;

    // Left & Right border
    if (ball.position.x - ball.radius) <= 0.0 || (ball.position.x + ball.radius) >= window_width {
        ball.speed.x = -ball.speed.x;
    }
    // Top border
    if (ball.position.y - ball.radius) <= 0.0 {
        ball.speed.y = -ball.speed.y;
    }
}

/// Checks if the [`Ball`] is touching the bottom window border
fn check_game_lose(ball: &Ball, context: &Context) -> bool{
    let window_height = context.gfx.window().inner_size().height as f32;
    
    (ball.position.y + ball.radius) >= window_height
}

/// Keep player on the screen
fn force_player_boundaries(player: &mut PlayerPaddle, context: &Context) {
    // Left Border
    if player.rect.x <= 0.0 {
        player.rect.x = 0.0;
        return;
    }

    // Right Border
    let window_width = context.gfx.window().inner_size().width as f32;
    if (player.rect.x + player.rect.w) >= window_width {
        player.rect.x = window_width - player.rect.w;
    }
}

const TARGET_UPS: u32 = 60;
const WINDOW_TITLE: &str = "breakout_rs";
// const BACKGROUND_COLOR: Color = Color::from_rgba(26, 128, 77, 255);
// const FPS_TEXT_COLOR: Color = Color::from_rgba()

fn main() {
    let (conf, (width, height)) = setup_conf();
    let (context, event_loop) = ContextBuilder::new(WINDOW_TITLE, "DocE")
        .default_conf(conf)
        .build()
        .expect("Building context failed!");

    let win_lose_rect_width = width / 3.0;
    let win_lose_rect_height = height / 5.0;
    let game_state = GameState {
        delta_time: std::time::Duration::new(0, 0),
        has_won: false,
        win_lose_rect: WinLoseRect {
            rect: Rect::new(
            (width / 2.0) - (win_lose_rect_width / 2.0),
            (height / 2.0) - (win_lose_rect_height / 2.0),
            win_lose_rect_width,
            win_lose_rect_height,
            ),
            radius: 32.0,
        },
        player: PlayerPaddle {
            rect: Rect::new(
                width / 10.0,
                height - (height / 10.0),
                width / 10.0,
                height / 20.0,
            ),
            speed: 10.0,
            direction: None,
        },
        ball: Ball {
            position: Point2 {
                x: width / 2.0,
                y: height / 2.0,
            },
            radius: 20.0,
            speed: Point2 {
                x: 7.0,
                y: 7.0,
            },
        },
    };

    event::run(context, event_loop, game_state);
}

fn setup_conf() -> (Conf, (f32, f32)) {
    let (width, height) = (800.0, 600.0);
    let (min_width, min_height) = (width, height);

    let conf = Conf {
        window_mode: WindowMode {
            width,
            height,
            min_width,
            min_height,
            ..WindowMode::default()
        },
        window_setup: WindowSetup {
            title: WINDOW_TITLE.to_owned(),
            icon: "".to_owned(),
            ..WindowSetup::default()
        },
        ..Conf::default()
    };

    (conf, (width, height))
}