use std::error::Error;

use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::WindowCanvas;

use crate::arena::arena::Arena;
use crate::data::cameras::Cameras;
use crate::data::diagnostics::{Diagnostic, Diagnostics};
use crate::data::player::Player;
use crate::engine::assets::image_asset::ImageAsset;
use crate::game_props::{
    AMBER_ACCENT, ANTIQUE_WHITE, HUD_DIAGNOSITCS_WIDTH_PERCENT, HUD_DIAGNOSTICS_HEIGHT,
    RENDER_GRID, TILE_SIZE,
};
use crate::inputs::input::Input;
use crate::views::floor_view::FloorView;
use crate::views::grid_view::GridView;
use crate::views::hud_diagnostics::HudDiagnostics;
use crate::views::player_view::PlayerView;
use crate::views::text::Text;
use crate::views::walls_view::WallsView;

pub struct Game<'a> {
    // views
    arena: &'a Arena,
    floor: FloorView<'a>,
    grid: GridView,
    walls: WallsView<'a>,
    hud_diagnostics: HudDiagnostics<'a>,
    player_view: PlayerView,

    // data
    diagnostics: Diagnostics,
    player: Player,

    cameras: Cameras,
}

impl<'a> Game<'a> {
    pub fn new(
        arena: &'a Arena,
        floor_asset: &'a ImageAsset<'a>,
        wall_asset: &'a ImageAsset<'a>,
        stats_text: Text<'a>,
    ) -> Result<Self, Box<dyn Error>> {
        // views
        let floor = FloorView::from_arena(&arena, floor_asset, TILE_SIZE);
        let grid = GridView::new(arena.ncols, arena.nrows, TILE_SIZE);
        let walls = WallsView::new(&arena.walls, wall_asset, TILE_SIZE);
        let hud_diagnostics = HudDiagnostics::new(
            Point::new(0, 0),
            HUD_DIAGNOSTICS_HEIGHT,
            HUD_DIAGNOSITCS_WIDTH_PERCENT,
            stats_text,
        );
        let player_view = PlayerView::new(Color::RGB.call(AMBER_ACCENT));

        // data
        let player = Player::new(&arena.player);

        // cameras
        let cameras = Cameras::new();
        Ok(Game {
            arena,
            floor,
            grid,
            walls,
            hud_diagnostics,
            player_view,
            diagnostics: Diagnostics::new(),
            player,
            cameras,
        })
    }

    pub fn update(
        &mut self,
        dt: f32,
        window_size: &(u32, u32),
        input: &Input,
        diagnostics: Diagnostic,
    ) {
        self.player_view.debug(true);
        self.player.update(dt, input);
        self.diagnostics.update(diagnostics);
        self.cameras.update(
            &self.player.tile_position.to_world_point(TILE_SIZE),
            &window_size,
        );
    }

    pub fn render(&self, canvas: &mut WindowCanvas) -> Result<(), Box<dyn Error>> {
        let window_size = canvas.window().drawable_size();

        canvas.set_draw_color(Color::RGB.call(ANTIQUE_WHITE));
        canvas.clear();
        if RENDER_GRID {
            self.grid.render(canvas)?;
        }
        self.floor.render(canvas, &self.cameras.platform)?;
        self.walls.render(canvas, &self.cameras.platform)?;
        self.hud_diagnostics
            .render(canvas, &self.diagnostics.current(), &window_size)?;
        self.player_view.render(canvas, &self.player)?;
        canvas.present();
        Ok(())
    }
}
