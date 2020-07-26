use crate::data::player::Player;
use ggez::graphics::{Color, DrawMode, DrawParam, MeshBuilder, Rect, StrokeOptions};
use ggez::nalgebra::Point2;
use ggez::{graphics, Context, GameResult};

pub struct PlayerView {
    debug_player_hit_tile: bool,
    player_hit_tile_color: Color,
    tile_size: u32,
}

impl PlayerView {
    pub fn new(player_hit_tile_color: Color, tile_size: u32) -> PlayerView {
        PlayerView {
            debug_player_hit_tile: true,
            player_hit_tile_color,
            tile_size,
        }
    }

    pub fn debug(&mut self, debug_player_hit_tile: bool) {
        self.debug_player_hit_tile = debug_player_hit_tile;
    }

    pub fn render(&self, ctx: &mut Context, viewport: &Rect, player: &Player) -> GameResult {
        let wp = player.tile_position.to_world_point(self.tile_size);
        let pos = Point2::new(wp.x - viewport.x, wp.y - viewport.y);
        self.render_debug_hit_tile(ctx, pos, player.radius)
    }

    fn render_debug_hit_tile(
        &self,
        ctx: &mut Context,
        pos: Point2<f32>,
        radius: f32,
    ) -> GameResult {
        if self.debug_player_hit_tile {
            let width = radius * 2.0;
            let player_rect = Rect::new(-radius, -radius, width, width);
            let mode = DrawMode::Stroke(StrokeOptions::default());

            let mut mesh_builder = MeshBuilder::new();
            mesh_builder.rectangle(mode, player_rect, self.player_hit_tile_color);
            let mesh = mesh_builder.build(ctx)?;
            let draw_param = DrawParam::default().dest(pos);
            graphics::draw(ctx, &mesh, draw_param)
        } else {
            Ok(())
        }
    }
}
