use sdl2::rect::{Point, Rect};

#[derive(Debug, PartialEq)]
pub struct TilePosition {
    pub col: u32,
    pub row: u32,
    pub rel_x: i32,
    pub rel_y: i32,
}

impl TilePosition {
    pub fn new(col: u32, row: u32, rel_x: i32, rel_y: i32) -> TilePosition {
        TilePosition {
            col,
            row,
            rel_x,
            rel_y,
        }
    }

    pub fn centered(col: u32, row: u32, tile_size: u32) -> TilePosition {
        let rel_x = (tile_size / 2) as i32;
        let rel_y = (tile_size / 2) as i32;
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
}

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
        let rel_x = self.x % tile_size;
        let rel_y = self.y % tile_size;
        TilePosition::new(col, row, rel_x as i32, rel_y as i32)
    }

    pub fn to_point(&self) -> Point {
        Point::new(self.x as i32, self.y as i32)
    }

    pub fn to_rect(&self, tile_size: u32) -> Rect {
        Rect::from_center(self.to_point(), tile_size, tile_size)
    }
}
