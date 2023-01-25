#![warn(clippy::unwrap_used)]

use ggez::*;
use ggez::conf::{Conf, WindowMode, WindowSetup};
use ggez::graphics::{Color, DrawParam, Quad, Rect, Text};

struct Paddle {
    rect: Rect,
    speed: i32,
}

struct Ball {
    rect: Rect,
    // image: Image, // TODO: cannot get Image to load
    // image: Image::from_path(&context, "/ltt-logo.png").expect("Loading paddle image failed!"),
}

struct GameState {
    delta_time: std::time::Duration,
    paddle: Paddle,
}

impl event::EventHandler<GameError> for GameState {
    fn update(&mut self, context: &mut Context) -> Result<(), GameError> {
        const TARGET_UPS: u32 = 60;

        while context.time.check_update_time(TARGET_UPS) {
            self.delta_time = context.time.delta();
        }

        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> Result<(), GameError> {
        let mut canvas = graphics::Canvas::from_frame(context, Color::from([0.1, 0.5, 0.3, 1.0]));

        // draw fps counter
        let fps = context.time.fps();
        let fps_display = Text::new(format!("FPS: {}", fps));
        canvas.draw(
            &fps_display,
            DrawParam::from([0.0, 0.0]).color(Color::WHITE),
        );

        // draw paddle
        let paddle_rect = &self.paddle.rect;
        canvas.draw(
            &Quad,
            DrawParam::new()
                .dest(paddle_rect.point())
                .scale(paddle_rect.size())
                .color(Color::WHITE),
        );

        canvas.finish(context)?;
        Ok(())
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
        paddle: Paddle {
            rect: Rect::new(
                width / 10.0,
                height - (height / 10.0),
                width / 10.0,
                height / 20.0,
            ),
            speed: 1,
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
