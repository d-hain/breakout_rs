#![warn(clippy::unwrap_used)]

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

struct Ball {
    position: Point2<f32>,
    radius: f32,
    speed: f32,
}

struct GameState {
    delta_time: std::time::Duration,
    player: PlayerPaddle,
    ball: Ball,
}

impl event::EventHandler<GameError> for GameState {
    fn update(&mut self, context: &mut Context) -> Result<(), GameError> {
        const TARGET_UPS: u32 = 60;

        while context.time.check_update_time(TARGET_UPS) {
            self.delta_time = context.time.delta();

            // move player
            match self.player.direction {
                Some(Direction::Left) => self.player.rect.x -= self.player.speed,
                Some(Direction::Right) => self.player.rect.x += self.player.speed,
                None => (),
            }
            self.player.direction = None;

            force_player_boundaries(&mut self.player, context);
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
            1.0,
            Color::WHITE,
        )?;
        let ball_mesh = graphics::Mesh::from_data(context, ball_mesh_builder.build());
        canvas.draw(&ball_mesh, DrawParam::new());

        canvas.finish(context)?;
        Ok(())
    }

    fn key_down_event(&mut self, _context: &mut Context, input: KeyInput, _repeated: bool) -> Result<(), GameError> {
        self.player.direction = Direction::from_scancode(input.scancode);

        Ok(())
    }
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

fn main() {
    let (conf, (width, height)) = setup_conf();
    let (context, event_loop) = ContextBuilder::new("breakout_rs", "DocE")
        .default_conf(conf)
        .build()
        .expect("Building context failed!");

    let game_state = GameState {
        delta_time: std::time::Duration::new(0, 0),
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
            speed: 10.0,
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
            resizable: true,
            ..WindowMode::default()
        },
        window_setup: WindowSetup {
            title: "breakout_rs".to_string(),
            icon: "".to_string(),
            ..WindowSetup::default()
        },
        ..Conf::default()
    };

    (conf, (width, height))
}
