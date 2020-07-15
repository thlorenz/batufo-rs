use crate::engine::position::TilePosition;
use crate::engine::vector::Vector;
use crate::game_props::TILE_SIZE;
use crate::inputs::input::Input;

pub struct Player {
    pub tile_position: TilePosition,
    pub radius: u32,
    pub velocity: Vector,
}

impl Player {
    pub fn new(tile_position: &TilePosition) -> Player {
        let radius = TILE_SIZE / 2;
        // let velocity = Vector::zero();
        let velocity = Vector::new(0.0, 0.0);
        Player {
            tile_position: tile_position.clone(),
            radius,
            velocity,
        }
    }

    pub fn update(&mut self, dt: f32, input: &Input) {
        let new_tp = self
            .tile_position
            .apply_velocity(dt, &self.velocity, TILE_SIZE);
        self.tile_position = new_tp;
        // TODO: need to have proper increase velocity function that takes thrust force
        // and angle into account (see vector.rs)
        if input.has_up() {
            self.velocity = self.velocity.translate(0.0, -0.2);
        }
        if input.has_right() {
            self.velocity = self.velocity.translate(0.2, 0.0);
        }
        if input.has_down() {
            self.velocity = self.velocity.translate(0.0, 0.2);
        }
        if input.has_left() {
            self.velocity = self.velocity.translate(-0.2, 0.0);
        }
    }
}
