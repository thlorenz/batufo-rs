use crate::arena::arena::Arena;
use crate::engine::position::TilePosition;

struct FloorRender<'a> {
    floor_tiles: &'a Vec<TilePosition>,
    tile_size: u32,
}

impl FloorRender<'_> {
    pub fn new(floor_tiles: &'static Vec<TilePosition>, tile_size: u32) -> Self {
        FloorRender {
            floor_tiles,
            tile_size,
        }
    }

    pub fn from_arena(arena: &'static Arena, tile_size: u32) -> FloorRender {
        FloorRender::new(&arena.floor_tiles, tile_size)
    }

    pub fn render(&self) {}

    fn init_tiles(&self) {
        /* TODO: continue here
        let mut i = 0;
        for tp in self.floor_tiles {}

         */
    }
}
