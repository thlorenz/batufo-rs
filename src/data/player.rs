use crate::engine::position::TilePosition;
use ggez::nalgebra::Vector2;

pub struct Player {
    pub tile_position: TilePosition,
    pub radius: u32,
    pub velocity: Vector2<f32>,
    tile_size: u32,
}

impl Player {
    pub fn new(tile_position: TilePosition, tile_size: u32) -> Player {
        let radius = tile_size / 2;
        let velocity = Vector2::new(0.0, 5.0);
        Player {
            tile_position,
            radius,
            velocity,
            tile_size,
        }
    }

    pub fn update(&mut self, dt: f32) {
        let new_tp = self
            .tile_position
            .apply_velocity(dt, &self.velocity, self.tile_size);
        self.tile_position = new_tp;
    }
}
