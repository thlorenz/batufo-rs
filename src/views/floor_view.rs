use crate::arena::arena::Arena;
use crate::engine::image_asset::ImageAsset;
use crate::engine::position::TilePosition;
use ggez::graphics::spritebatch::SpriteBatch;
use ggez::graphics::DrawParam;
use ggez::graphics::Rect;
use ggez::nalgebra::Point2;
use ggez::{graphics, Context, GameResult};

struct FloorTile {
    center: Point2<f32>,
    tile_idx: u32,
}

pub struct FloorView {
    sprite_batch: SpriteBatch,
}

impl FloorView {
    pub fn new(asset: ImageAsset, tile_positions: &Vec<TilePosition>, tile_size: u32) -> FloorView {
        let floor_tiles = init_floor_tiles(&asset, tile_positions, tile_size);
        let mut sprite_batch = SpriteBatch::new(asset.texture.clone());
        let ht = (tile_size / 2) as f32;
        for tile in &floor_tiles {
            let src = asset.rect_for_idx(tile.tile_idx);
            let dst = Point2::new(tile.center.coords.x - ht, tile.center.y - ht);
            let param = DrawParam::default().src(src).dest(dst);
            sprite_batch.add(param);
        }
        FloorView { sprite_batch }
    }

    pub fn from_arena(asset: ImageAsset, arena: &Arena, tile_size: u32) -> FloorView {
        FloorView::new(asset, &arena.floor_tiles, tile_size)
    }

    pub fn render(&self, ctx: &mut Context, viewport: &Rect) -> GameResult {
        let dst = Point2::new(-viewport.left(), -viewport.top());
        let param = (dst, 0.0, graphics::WHITE);
        graphics::draw(ctx, &self.sprite_batch, param)
    }
}

fn init_floor_tiles(
    asset: &ImageAsset,
    tile_positions: &Vec<TilePosition>,
    tile_size: u32,
) -> Vec<FloorTile> {
    let mut i = 0;
    tile_positions
        .iter()
        .map(|tp| {
            let center = tp.to_world_point(tile_size);
            let row = i % 7;
            let col = (i / asset.rows) % 7;
            let tile_idx = row * asset.cols + col;
            i = i + 1;
            FloorTile { center, tile_idx }
        })
        .collect()
}
