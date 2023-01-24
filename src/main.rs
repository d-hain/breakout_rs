use ggez::{conf, Context, ContextBuilder, GameError};

struct GameState {
    delta_time: std::time::Duration,
}

impl ggez::event::EventHandler<GameError> for GameState {
    fn update(&mut self, context: &mut Context) -> Result<(), GameError> {
        self.delta_time = context.time.delta();
        
        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> Result<(), GameError> {
        let seconds = self.delta_time.as_secs_f32();
        println!("Hello, breakout! dt={}s", seconds);
        
        Ok(())
    }
}


fn main() {
    let state = GameState {
        delta_time: std::time::Duration::new(0, 0),
    };
    let conf = conf::Conf::new();
    let (context, event_loop) = ContextBuilder::new("breakout_rs", "DocE")
        .default_conf(conf)
        .build()
        .unwrap();
    
    ggez::event::run(context, event_loop, state);
}
