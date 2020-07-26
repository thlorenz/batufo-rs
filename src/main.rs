mod arena;
mod engine;
mod game_props;
mod views;

use std::env;
use std::path;

use crate::arena::arena::Arena;
use crate::engine::image_asset::ImageAsset;
use crate::game_props::{ANTIQUE_WHITE, TILE_SIZE};
use crate::views::floor_view::FloorView;
use crate::views::grid_view::GridView;
use ggez;
use ggez::event;
use ggez::graphics;
use ggez::graphics::Image;
use ggez::{Context, GameResult};

struct GameState {
    frames: usize,
    floor_view: FloorView,
    grid_view: GridView,
    font: graphics::Font,
}

impl GameState {
    fn new(ctx: &mut Context, arena: Arena) -> GameResult<GameState> {
        let font = graphics::Font::new(ctx, "/fonts/RobotoMono.ttf")?;
        let floor_view = init_floor_view(ctx, &arena)?;
        let grid_view = GridView::new(ctx, arena.ncols, arena.nrows, TILE_SIZE)?;

        let s = GameState {
            frames: 0,
            font,
            floor_view,
            grid_view,
        };
        Ok(s)
    }
}

fn init_floor_view(ctx: &mut Context, arena: &Arena) -> ggez::GameResult<FloorView> {
    let texture = Image::new(ctx, "/images/bg/floor-tiles.png")?;
    let image_asset = ImageAsset::new(384, 384, 8, 8, texture);
    Ok(FloorView::from_arena(image_asset, arena, TILE_SIZE))
}

impl event::EventHandler for GameState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, ANTIQUE_WHITE.into());

        self.grid_view.render(ctx)?;
        self.floor_view.render(ctx, game_props::USE_SPRITE_BATCH)?;
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

    let arena = Arena::for_level("face off").expect("FATAL: unable to create arena");
    let state = &mut GameState::new(ctx, arena)?;
    event::run(ctx, event_loop, state)
}
