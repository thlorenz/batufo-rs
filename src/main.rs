mod arena;
mod engine;
mod game_props;
mod views;

use std::env;
use std::path;

use cgmath;
use ggez;
use ggez::event;
use ggez::graphics;
use ggez::graphics::Color;
use ggez::{Context, GameResult};

struct GameState {
    frames: usize,
    font: graphics::Font,
}

impl GameState {
    fn new(ctx: &mut Context) -> GameResult<GameState> {
        let font = graphics::Font::new(ctx, "/fonts/RobotoMono.ttf")?;

        let s = GameState { frames: 0, font };
        Ok(s)
    }
}

impl event::EventHandler for GameState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, Color::from_rgb(0xaa, 0xaa, 0xaa));

        let offset = self.frames as f32 / 10.0;
        let dest_point = cgmath::Point2::new(offset, offset);
        let text = graphics::Text::new((
            format!("Frame: {}, Hello world! at {}", self.frames, offset),
            self.font,
            48.0,
        ));

        graphics::draw(ctx, &text, (dest_point, Color::from_rgb(255, 0, 0)))?;
        graphics::present(ctx)?;

        self.frames += 1;
        if (self.frames % 100) == 0 {
            let dt = ggez::timer::delta(ctx).as_millis();
            let fps = ggez::timer::fps(ctx);
            eprintln!("FPS: {}, dt: {}", fps, dt);
        }

        Ok(())
    }
}

pub fn main() -> GameResult {
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    let context_builder = ggez::ContextBuilder::new("batufo", "Thorsten Lorenz")
        .with_conf_file(true)
        .add_resource_path(resource_dir);
    let (ctx, event_loop) = &mut context_builder.build()?;

    let state = &mut GameState::new(ctx)?;
    event::run(ctx, event_loop, state)
}
