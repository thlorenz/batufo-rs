use sdl2::rect::{Point, Rect};
use sdl2::render::{Texture, TextureValueError, WindowCanvas};

use crate::engine::assets::image_asset::ImageAsset;
use crate::game_props::TILE_SIZE;

fn rect_visible(rect: Rect) -> bool {
    // Just TILE_SIZE is not enough, however this is the smallest that worked without
    // showing the freeze for the face_off level.
    let size = TILE_SIZE as i32 * 2;

    // TODO: in the future we should take the actual viewport into account as well
    let p = rect.bottom_left();
    p.x > size && p.y > size
}

pub struct Sprite<'a> {
    texture: &'a Texture<'a>,
    rect: Rect,
    render_size: u32,
}

impl<'a> Sprite<'a> {
    pub fn new(
        asset: &'a ImageAsset,
        render_size: u32,
        rect_idx: u32,
    ) -> Result<Sprite<'a>, TextureValueError> {
        let rect = asset.rect_for_idx(rect_idx);
        Ok(Sprite {
            texture: &asset.texture,
            rect,
            render_size,
        })
    }

    pub fn render(&self, canvas: &mut WindowCanvas, rect: Rect) -> Result<(), String> {
        if rect_visible(rect) {
            canvas.copy(&self.texture, Some(self.rect), Some(rect))?
        }
        Ok(())
    }

    pub fn render_centered(&self, canvas: &mut WindowCanvas, center: Point) -> Result<(), String> {
        let rect = Rect::from_center(center, self.render_size, self.render_size);
        self.render(canvas, rect)
    }
}

pub struct PositionedSprite<'a> {
    sprite: Sprite<'a>,
    center: Point,
}

impl<'a> PositionedSprite<'a> {
    pub fn new(sprite: Sprite<'a>, center: Point) -> Self {
        PositionedSprite { sprite, center }
    }

    pub fn render(&self, canvas: &mut WindowCanvas, viewport: &Rect) -> Result<(), String> {
        let center = self.center - viewport.top_left();
        self.sprite.render_centered(canvas, center)
    }
}
