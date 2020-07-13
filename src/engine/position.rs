use sdl2::rect::{Point, Rect};

use crate::engine::vector::Vector;

#[derive(Clone, Debug, PartialEq)]
pub struct TilePosition {
    pub col: u32,
    pub row: u32,
    pub rel_x: f32,
    pub rel_y: f32,
}

impl TilePosition {
    pub fn new(col: u32, row: u32, rel_x: f32, rel_y: f32) -> TilePosition {
        TilePosition {
            col,
            row,
            rel_x,
            rel_y,
        }
    }

    pub fn centered(col: u32, row: u32, tile_size: u32) -> TilePosition {
        let rel_x = tile_size as f32 / 2.0;
        let rel_y = tile_size as f32 / 2.0;
        TilePosition::new(col, row, rel_x, rel_y)
    }

    pub fn to_world_position(&self, tile_size: u32) -> WorldPosition {
        WorldPosition::from_tile_position(self, tile_size)
    }

    pub fn to_world_point(&self, tile_size: u32) -> Point {
        WorldPosition::from_tile_position(self, tile_size).to_point()
    }

    pub fn to_world_rect(&self, tile_size: u32) -> Rect {
        WorldPosition::from_tile_position(self, tile_size).to_rect(tile_size)
    }

    pub fn apply_velocity(&self, dt: f32, velocity: &Vector, tile_size: u32) -> TilePosition {
        if *velocity == Vector::zero() {
            return self.clone();
        }
        let wp = self.to_world_position(tile_size);
        let dx = wp.x + (velocity.x * dt) as u32;
        let dy = wp.y + (velocity.y * dt) as u32;
        WorldPosition::new(dx, dy).to_tile_position(tile_size)
    }
}

#[derive(Debug, PartialEq)]
pub struct WorldPosition {
    pub x: u32,
    pub y: u32,
}

impl WorldPosition {
    pub fn new(x: u32, y: u32) -> WorldPosition {
        WorldPosition { x, y }
    }

    pub fn from_tile_position(tp: &TilePosition, tile_size: u32) -> WorldPosition {
        let rel = tile_size / 2;
        let x = tile_size * tp.col + rel;
        let y = tile_size * tp.row + rel;
        WorldPosition::new(x, y)
    }

    pub fn to_tile_position(&self, tile_size: u32) -> TilePosition {
        let col = self.x / tile_size;
        let row = self.y / tile_size;
        let rel_x = (self.x % tile_size) as f32;
        let rel_y = (self.y % tile_size) as f32;
        TilePosition::new(col, row, rel_x, rel_y)
    }

    pub fn to_point(&self) -> Point {
        Point::new(self.x as i32, self.y as i32)
    }

    pub fn to_rect(&self, tile_size: u32) -> Rect {
        Rect::from_center(self.to_point(), tile_size, tile_size)
    }
}

#[cfg(test)]
mod tests {
    const TILE_SIZE: u32 = 20;
    use super::*;
    mod tile_position {
        use super::*;

        #[test]
        fn init() {
            let tp = TilePosition::new(1, 1, 10.0, 10.0);
            let centered = TilePosition::centered(1, 1, TILE_SIZE);
            assert_eq!(tp, centered, "new(1, 1, 10.0, 10.0) == centered(1, 1, 20)")
        }

        #[test]
        fn conversions() {
            let tp = TilePosition::new(10, 10, 10.0, 10.0);
            assert_eq!(
                tp.to_world_position(TILE_SIZE),
                WorldPosition::new(210, 210),
                "to_world_position"
            );
            assert_eq!(
                tp.to_world_point(TILE_SIZE),
                Point::new(210, 210),
                "to_world_point"
            );
            assert_eq!(
                tp.to_world_rect(TILE_SIZE),
                Rect::new(200, 200, TILE_SIZE, TILE_SIZE),
                "to_world_rect"
            );
        }
    }
}
