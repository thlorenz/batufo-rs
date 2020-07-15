use sdl2::rect::{Point, Rect};
use sdl2::render::{Texture, TextureValueError, WindowCanvas};

use crate::engine::assets::image_asset::ImageAsset;

fn rect_visible(rect: &Rect, viewport: &Rect) -> bool {
    viewport.contains_point(rect.top_left())
        || viewport.contains_point(rect.top_right())
        || viewport.contains_point(rect.bottom_right())
        || viewport.contains_point(rect.bottom_left())
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

    pub fn render(
        &self,
        canvas: &mut WindowCanvas,
        viewport: &Rect,
        rect: &Rect,
    ) -> Result<(), String> {
        let adjusted_to_viewport = Rect::new(
            rect.x - viewport.x,
            rect.y - viewport.y,
            rect.width(),
            rect.height(),
        );
        if rect_visible(rect, viewport) {
            canvas.copy(&self.texture, Some(self.rect), Some(adjusted_to_viewport))?
        }
        Ok(())
    }

    pub fn render_centered(
        &self,
        canvas: &mut WindowCanvas,
        viewport: &Rect,
        center: Point,
    ) -> Result<(), String> {
        let rect = Rect::from_center(center, self.render_size, self.render_size);
        self.render(canvas, viewport, &rect)
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
        self.sprite.render_centered(canvas, viewport, self.center)
    }
}
