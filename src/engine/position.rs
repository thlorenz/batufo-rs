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
        // TODO: due to rounding we're loosing info when velocity is very small
        // it seems like we'd need to track world position as floats and only
        // convert to pixels when drawing
        let dx = (velocity.x * dt).round() as i32;
        let dy = (velocity.y * dt).round() as i32;
        let x = (wp.x as i32 + dx).max(0) as u32;
        let y = (wp.y as i32 + dy).max(0) as u32;
        let new_wp = WorldPosition::new(x, y);
        /*
        eprintln!(
            "({}) {:?} {:?} + ({}:{}) -> {:?}",
            dt, velocity, wp, dx, dy, new_wp
        );

         */
        new_wp.to_tile_position(tile_size)
    }
}

#[derive(Debug, PartialEq)]
// TODO: should world position have negative coordinates?
//   how else are we gonna draw background that's outside the tilemap?
pub struct WorldPosition {
    pub x: u32,
    pub y: u32,
}

impl WorldPosition {
    pub fn new(x: u32, y: u32) -> WorldPosition {
        WorldPosition { x, y }
    }

    pub fn from_tile_position(tp: &TilePosition, tile_size: u32) -> WorldPosition {
        let x = tile_size * tp.col + tp.rel_x.round() as u32;
        let y = tile_size * tp.row + tp.rel_y.round() as u32;
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
    use super::*;

    const TILE_SIZE: u32 = 20;

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

        #[test]
        fn tile_world_round_trips() {
            let wp0 = WorldPosition { x: 210, y: 240 };
            let tp = wp0.to_tile_position(TILE_SIZE);
            let wp1 = tp.to_world_position(TILE_SIZE);
            assert_eq!(wp0, wp1);

            let wp0 = WorldPosition { x: 240, y: 241 };
            let tp = wp0.to_tile_position(TILE_SIZE);
            let wp1 = tp.to_world_position(TILE_SIZE);
            assert_eq!(wp0, wp1);

            let wp0 = WorldPosition { x: 10, y: 21 };
            let tp = wp0.to_tile_position(5);
            let wp1 = tp.to_world_position(5);
            assert_eq!(wp0, wp1);
        }

        mod apply_velocity_tile_size_on_y {
            use super::*;
            #[test]
            fn dt_1_0() {
                let tp0 = TilePosition::new(10, 10, 10.0, 10.0);
                let velocity = Vector::new(0.0, TILE_SIZE as f32);

                let dt = 1.0;
                let tp1 = tp0.apply_velocity(dt, &velocity, TILE_SIZE);
                let tp2 = tp1.apply_velocity(dt, &velocity, TILE_SIZE);
                assert_eq!(
                    tp1,
                    TilePosition {
                        col: 10,
                        row: 11,
                        rel_x: 10.0,
                        rel_y: 10.0,
                    }
                );
                assert_eq!(
                    tp2,
                    TilePosition {
                        col: 10,
                        row: 12,
                        rel_x: 10.0,
                        rel_y: 10.0,
                    }
                );
            }

            #[test]
            fn dt_1_5() {
                let tp0 = TilePosition::new(10, 10, 10.0, 10.0);
                let velocity = Vector::new(0.0, TILE_SIZE as f32);

                let dt = 1.5;
                let tp1 = tp0.apply_velocity(dt, &velocity, TILE_SIZE);
                let tp2 = tp1.apply_velocity(dt, &velocity, TILE_SIZE);

                assert_eq!(
                    tp1,
                    TilePosition {
                        col: 10,
                        row: 12,
                        rel_x: 10.0,
                        rel_y: 0.0,
                    }
                );
                assert_eq!(
                    tp2,
                    TilePosition {
                        col: 10,
                        row: 13,
                        rel_x: 10.0,
                        rel_y: 10.0,
                    }
                );
            }

            #[test]
            fn dt_16_velocity_neg_y() {
                let tp0 = TilePosition::new(10, 10, 10.0, 10.0);
                let velocity = Vector::new(0.0, -1.0);
                let dt = 16.0;
                let tp1 = tp0.apply_velocity(dt, &velocity, TILE_SIZE);
                assert_eq!(
                    tp1,
                    TilePosition {
                        col: 10,
                        row: 9,
                        rel_x: 10.0,
                        rel_y: 14.0
                    }
                );
            }
        }
    }
}
