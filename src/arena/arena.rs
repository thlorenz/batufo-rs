use crate::arena::levels::Levels;
use crate::arena::tilemap::{needs_floor_tile, Tilemap};
use crate::engine::position::TilePosition;
use crate::game_props::TILE_SIZE;
use std::error::Error;
use std::fmt;

#[derive(fmt::Debug)]
pub struct Arena {
    pub floor_tiles: Vec<TilePosition>,
    pub ncols: u32,
    pub nrows: u32,
}

impl Arena {
    pub fn new(floor_tiles: Vec<TilePosition>, ncols: u32, nrows: u32) -> Arena {
        Arena {
            floor_tiles,
            ncols,
            nrows,
        }
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
        Arena::new(floor_tiles, ncols, nrows)
    }

    fn for_level(level_name: &'static str) -> Result<Arena, Box<dyn Error>> {
        let levels = Levels::new();
        let face_off = levels
            .get_level(level_name)
            .ok_or(format!("level not found '{}'", level_name))?;
        let tilemap = Tilemap::new(face_off.level_string, TILE_SIZE)?;
        Ok(Arena::from_tilemap(tilemap))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TILE_SIZE: u32 = 2;
    const CENTER: i32 = (TILE_SIZE / 2) as i32;
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
