use crate::engine::assets::image_asset::ImageAsset;
use sdl2::render::WindowCanvas;

use crate::arena::arena::Arena;
use crate::data::diagnostics::{Diagnostic, Diagnostics};
use crate::data::player::Player;
use crate::game_props::{AMBER_ACCENT, ANTIQUE_WHITE, RENDER_GRID, TILE_SIZE};
use crate::inputs::input::Input;
use crate::views::diag_hud::DiagHud;
use crate::views::floor_view::FloorView;
use crate::views::grid_view::GridView;
use crate::views::player_view::PlayerView;
use crate::views::text::Text;
use crate::views::walls_view::WallsView;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use std::error::Error;

pub struct Game<'a> {
    // views
    arena: &'a Arena,
    floor: FloorView<'a>,
    grid: GridView,
    walls: WallsView<'a>,
    diag_hud: DiagHud<'a>,
    player_view: PlayerView,

    // data
    diagnostics: Diagnostics,
    player: Player,

    // cameras
    camera_platform: Point,
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
        let diag_hud = DiagHud::new(Point::new(0, 0), stats_text);
        let player_view = PlayerView::new(Color::RGB.call(AMBER_ACCENT));

        // data
        let player = Player::new(&arena.player);

        // cameras
        let camera_platform = Point::new(0, 0);
        Ok(Game {
            arena,
            floor,
            grid,
            walls,
            diag_hud,
            player_view,
            diagnostics: Diagnostics::new(),
            player,
            camera_platform,
        })
    }

    pub fn update(&mut self, dt: f32, input: &Input, diagnostics: Diagnostic) {
        self.player_view.debug(true);
        self.player.update(dt, input);
        self.diagnostics.update(diagnostics);
    }

    pub fn render(&self, canvas: &mut WindowCanvas) -> Result<(), Box<dyn Error>> {
        canvas.set_draw_color(Color::RGB.call(ANTIQUE_WHITE));
        canvas.clear();
        if RENDER_GRID {
            self.grid.render(canvas)?;
        }
        self.floor.render(canvas, &self.camera_platform)?;
        self.walls.render(canvas, &self.camera_platform)?;
        self.diag_hud.render(canvas, &self.diagnostics.current())?;
        self.player_view.render(canvas, &self.player)?;
        canvas.present();
        Ok(())
    }
}
