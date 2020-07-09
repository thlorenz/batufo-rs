use crate::arena::arena::Arena;
use crate::engine::tile_position::TilePosition;

struct FloorTiles<'a> {
    floor_tiles: &'a Vec<TilePosition>,
}

impl FloorTiles<'_> {
    pub fn new(floor_tiles: &'static Vec<TilePosition>) -> Self {
        FloorTiles { floor_tiles }
    }

    pub fn from_arena(arena: &'static Arena) -> FloorTiles {
        FloorTiles::new(&arena.floor_tiles)
    }

    pub fn render(&self) {}
}
