#![warn(clippy::unwrap_used)]

use ggez::*;
use ggez::conf::{Conf, WindowMode, WindowSetup};
use ggez::graphics::{Color, DrawParam, Quad, Rect, Text};

#[derive(Copy, Clone)]
struct Window {
    width: f32,
    height: f32,
    min_width: f32,
    min_height: f32,
}

struct Paddle {
    rect: Rect,
    speed: i32,
}

struct Ball {
    rect: Rect,
    // image: Image, // TODO: cannot get Image to load
    // image: Image::from_path(&context, "/ltt-logo.png").expect("Loading paddle image failed!"),
    //             //TODO: cannot get Image to load
}

struct GameState {
    delta_time: std::time::Duration,
    window: Window,
    paddle: Paddle,
}

impl event::EventHandler<GameError> for GameState {
    fn update(&mut self, context: &mut Context) -> Result<(), GameError> {
        const TARGET_FPS: u8 = 60;
        
        while context.time.check_update_time(TARGET_FPS as u32) {
            self.delta_time = context.time.delta();
            //TODO: update gameState window height and width in if expression
        }
        
        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> Result<(), GameError> {
        let mut canvas = graphics::Canvas::from_frame(context, Color::from([0.1, 0.5, 0.3, 1.0]));
            
        // TODO: fixed framerate
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
    let (conf, window) = setup_conf();
    let (context, event_loop) = ContextBuilder::new("breakout_rs", "DocE")
        .default_conf(conf)
        .build()
        .expect("Building context failed!");

    let game_state = GameState {
        delta_time: std::time::Duration::new(0, 0),
        window,
        paddle: Paddle {
            rect: Rect::new(
                window.width / 10.0, 
                window.height - (window.height / 10.0), 
                window.width / 10.0, 
                window.height / 20.0,
            ),
            speed: 1,
        },
    };

    event::run(context, event_loop, game_state);
}

fn setup_conf() -> (Conf, Window) {
    let window = Window {
        width: 800.0,
        height: 600.0,
        min_width: 800.0,
        min_height: 600.0,
    };

    let conf = Conf {
        window_mode: WindowMode {
            width: window.width,
            height: window.height,
            min_width: window.min_width,
            min_height: window.min_height,
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

    (conf, window)
}
