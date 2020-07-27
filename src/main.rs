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
use crate::game_props::GameProps;
use crate::views::floor_view::FloorView;
use crate::views::grid_view::GridView;
use crate::views::player_view::PlayerView;
use ggez::event::KeyMods;
use ggez::graphics::Image;
use ggez::input::keyboard::KeyCode;
use ggez::{event, timer};
use ggez::{graphics, ContextBuilder};
use ggez::{Context, GameResult};

struct GameState {
    frames: usize,
    floor_view: FloorView,
    grid_view: GridView,
    player_view: PlayerView,

    cameras: Cameras,
    player: Player,

    game_props: GameProps,

    #[allow(dead_code)]
    font: graphics::Font,
}

impl GameState {
    fn new(ctx: &mut Context, arena: Arena, game_props: GameProps) -> GameResult<GameState> {
        let font = graphics::Font::new(ctx, "/fonts/RobotoMono.ttf")?;
        let floor_view = init_floor_view(ctx, &arena, game_props.tile_size)?;
        let grid_view = GridView::new(
            ctx,
            arena.ncols,
            arena.nrows,
            game_props.tile_size,
            game_props.colors.grid_color.into(),
        )?;
        let player_view = PlayerView::new(
            game_props.colors.player_hit_tile_color.into(),
            game_props.tile_size,
        );

        let cameras = Cameras::new(game_props.platform_lerp, game_props.planets_front_lerp);
        let player = Player::new(arena.player, game_props.tile_size);

        let s = GameState {
            frames: 0,
            font,
            floor_view,
            grid_view,
            player_view,

            cameras,
            player,

            game_props,
        };
        Ok(s)
    }
}

fn init_floor_view(ctx: &mut Context, arena: &Arena, tile_size: u32) -> GameResult<FloorView> {
    let texture = Image::new(ctx, "/images/bg/floor-tiles.png")?;
    let image_asset = ImageAsset::new(384, 384, 8, 8, texture);
    Ok(FloorView::from_arena(image_asset, arena, tile_size))
}

impl event::EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let gp = &self.game_props;
        // TODO: get this only on window resize in case it is expensive
        let win = graphics::window(ctx);
        let win_size = win.get_inner_size().unwrap();
        while timer::check_update_time(ctx, gp.physics_simulation_fps) {
            self.player.update(gp.physics_delta_time);
            self.cameras.update(
                gp.physics_delta_time,
                self.player.tile_position.to_world_point(gp.tile_size),
                &win_size.into(),
            );
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let gp = &self.game_props;
        graphics::clear(ctx, gp.colors.antique_white.into());

        self.grid_view
            .render(ctx, self.cameras.planets_front_origin())?;
        self.floor_view
            .render(ctx, self.cameras.platform_origin())?;
        self.player_view
            .render(ctx, self.cameras.platform_origin(), &self.player)?;
        graphics::present(ctx)?;

        self.frames += 1;
        if (self.frames % 100) == 0 {
            let dt = timer::delta(ctx).as_millis();
            let fps = timer::fps(ctx);
            eprintln!("FPS: {}, dt: {}", fps, dt);
        }

        Ok(())
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: KeyCode,
        _keymods: KeyMods,
        _repeat: bool,
    ) {
        let gp = &self.game_props;
        match keycode {
            KeyCode::W => self.player.accelerate([0.0, -gp.thrust_acceleration]),
            KeyCode::D => self.player.accelerate([gp.thrust_acceleration, 0.0]),
            KeyCode::S => self.player.accelerate([0.0, gp.thrust_acceleration]),
            KeyCode::A => self.player.accelerate([-gp.thrust_acceleration, 0.0]),
            KeyCode::Escape => ctx.continuing = false,
            _ => {}
        };
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

    let game_props = GameProps::default();
    game_props.to_toml();
    let arena =
        Arena::for_level("face off", game_props.tile_size).expect("FATAL: unable to create arena");
    let state = &mut GameState::new(ctx, arena, game_props)?;
    event::run(ctx, event_loop, state)
}
