use sdl2::rect::{Point, Rect};
use sdl2::render::{Texture, TextureValueError, WindowCanvas};

use crate::engine::assets::image_asset::ImageAsset;

pub struct Sprite<'a> {
    texture: &'a Texture<'a>,
    rect: Rect,
    render_size: u32,
}

impl<'a> Sprite<'a> {
    pub fn new(
        texture: &'a Texture<'a>,
        asset: &ImageAsset,
        render_size: u32,
        rect_idx: u32,
    ) -> Result<Sprite<'a>, TextureValueError> {
        let rect = asset.rect_for_idx(rect_idx);
        Ok(Sprite {
            texture,
            rect,
            render_size,
        })
    }

    pub fn render(&self, canvas: &mut WindowCanvas, center: Point) -> Result<(), String> {
        let rect = Rect::from_center(center, self.render_size, self.render_size);
        canvas.copy_ex(
            &self.texture,
            Some(self.rect),
            Some(rect),
            0.0,
            None,
            false,
            false,
        )
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

    pub fn render(&self, canvas: &mut WindowCanvas) -> Result<(), String> {
        self.sprite.render(canvas, self.center)
    }
}
