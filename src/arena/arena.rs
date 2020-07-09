use crate::arena::tilemap::{needs_floor_tile, Tilemap};
use crate::engine::tile_position::TilePosition;
use std::fmt;

#[derive(fmt::Debug)]
pub struct Arena {
    pub floor_tiles: Vec<TilePosition>,
}

impl Arena {
    pub fn new(floor_tiles: Vec<TilePosition>) -> Arena {
        Arena { floor_tiles }
    }

    pub fn from_tilemap(tilemap: Tilemap) -> Arena {
        let nrows = tilemap.nrows;
        let ncols = tilemap.ncols;
        let mut floor_tiles: Vec<TilePosition> = Vec::new();
        for row in 0..nrows {
            for col in 0..ncols {
                let idx: usize = (row * ncols + col) as usize;
                let tile = tilemap.tiles.get(idx).expect("should have tile at idx");
                if needs_floor_tile(tile) {
                    floor_tiles.push(TilePosition::centered(col, row, tilemap.tile_size))
                }
            }
        }
        Arena::new(floor_tiles)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    const TILE_SIZE: f32 = 1.0;
    const CENTER: f32 = TILE_SIZE / 2.0;
    #[test]
    fn floor_tiles() {
        let small_terrain = "
====
=  =
====
";

        let tilemap =
            Tilemap::new(small_terrain, TILE_SIZE).expect("should return correct tilemap");
        let arena = Arena::from_tilemap(tilemap);
        let floor_tiles = &arena.floor_tiles;
        assert_eq!(floor_tiles.len(), 2, "has two floor tiles");

        let tile0 = floor_tiles.get(0).unwrap();
        let tile1 = floor_tiles.get(1).unwrap();
        assert_eq!(tile0, &TilePosition::new(1, 1, CENTER, CENTER), "tile0");
        assert_eq!(tile1, &TilePosition::new(2, 1, CENTER, CENTER), "tile1");

        print!("{:?}", arena)
    }
}
