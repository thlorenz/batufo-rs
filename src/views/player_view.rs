use std::error::Error;

use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::WindowCanvas;

use crate::data::player::Player;
use crate::game_props::TILE_SIZE;

pub struct PlayerView {
    // sprite: Sprite<'a>,
    debug_player_hit_tile: bool,
    player_hit_tile_color: Color,
}

impl PlayerView {
    pub fn new(player_hit_tile_color: Color) -> PlayerView {
        PlayerView {
            debug_player_hit_tile: false,
            player_hit_tile_color,
        }
    }

    pub fn debug(&mut self, debug_player_hit_tile: bool) {
        self.debug_player_hit_tile = debug_player_hit_tile;
    }

    pub fn render(
        &self,
        canvas: &mut WindowCanvas,
        viewport: &Rect,
        player: &Player,
    ) -> Result<(), Box<dyn Error>> {
        let pos = player.tile_position.to_world_point(TILE_SIZE);
        let pos = pos - viewport.top_left();
        self.render_debug_hit_tile(canvas, pos, player.radius as i32)?;

        Ok(())
    }

    fn render_debug_hit_tile(
        &self,
        canvas: &mut WindowCanvas,
        pos: Point,
        radius: i32,
    ) -> Result<(), Box<dyn Error>> {
        if self.debug_player_hit_tile {
            let width = (radius * 2) as u32;
            let rect = Rect::new(pos.x - radius, pos.y - radius, width, width);
            canvas.set_draw_color(self.player_hit_tile_color);
            canvas.fill_rect(rect)?;
        }
        Ok(())
    }
}
