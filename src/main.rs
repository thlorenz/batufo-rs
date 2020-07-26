mod arena;
mod data;
mod engine;
mod game_props;
mod views;

use std::env;
use std::path;

use crate::arena::arena::Arena;
use crate::data::cameras::Cameras;
use crate::data::player::Player;
use crate::engine::image_asset::ImageAsset;
use crate::engine::position::TilePosition;
use crate::game_props::{ANTIQUE_WHITE, PHYSICS_DELTA_TIME, PHYSICS_SIMULATION_FPS, TILE_SIZE};
use crate::views::floor_view::FloorView;
use crate::views::grid_view::GridView;
use ggez::graphics::Image;
use ggez::{event, timer};
use ggez::{graphics, ContextBuilder};
use ggez::{Context, GameResult};

struct GameState {
    frames: usize,
    floor_view: FloorView,
    grid_view: GridView,

    cameras: Cameras,
    player: Player,

    #[allow(dead_code)]
    font: graphics::Font,
}

impl GameState {
    fn new(ctx: &mut Context, arena: Arena) -> GameResult<GameState> {
        let font = graphics::Font::new(ctx, "/fonts/RobotoMono.ttf")?;
        let floor_view = init_floor_view(ctx, &arena)?;
        let grid_view = GridView::new(ctx, arena.ncols, arena.nrows, TILE_SIZE)?;
        let ht = TILE_SIZE as f32 / 2.0;

        let cameras = Cameras::new();
        let player = Player::new(TilePosition::new(2, 2, ht, ht), TILE_SIZE);

        let s = GameState {
            frames: 0,
            font,
            floor_view,
            grid_view,

            cameras,
            player,
        };
        Ok(s)
    }
}

fn init_floor_view(ctx: &mut Context, arena: &Arena) -> GameResult<FloorView> {
    let texture = Image::new(ctx, "/images/bg/floor-tiles.png")?;
    let image_asset = ImageAsset::new(384, 384, 8, 8, texture);
    Ok(FloorView::from_arena(image_asset, arena, TILE_SIZE))
}

impl event::EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        while timer::check_update_time(ctx, PHYSICS_SIMULATION_FPS) {
            self.player.update(PHYSICS_DELTA_TIME);
            self.cameras.update(
                self.player.tile_position.to_world_point(TILE_SIZE),
                &(1600.0, 1200.0),
            );
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, ANTIQUE_WHITE.into());

        self.grid_view.render(ctx)?;
        self.floor_view.render(ctx, &self.cameras.platform)?;
        graphics::present(ctx)?;

        self.frames += 1;
        if (self.frames % 100) == 0 {
            let dt = timer::delta(ctx).as_millis();
            let fps = timer::fps(ctx);
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

    let context_builder = ContextBuilder::new("batufo", "Thorsten Lorenz")
        .with_conf_file(true)
        .add_resource_path(resource_dir);
    let (ctx, event_loop) = &mut context_builder.build()?;

    let arena = Arena::for_level("face off").expect("FATAL: unable to create arena");
    let state = &mut GameState::new(ctx, arena)?;
    event::run(ctx, event_loop, state)
}
