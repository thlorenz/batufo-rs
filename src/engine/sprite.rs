use sdl2::rect::{Point, Rect};
use sdl2::render::{Texture, TextureCreator, TextureValueError, WindowCanvas};
use sdl2::video::WindowContext;

use crate::engine::assets::image_asset::ImageAsset;

struct Sprite {
    texture: Texture<'static>,
    rect: Rect,
}

impl Sprite {
    pub fn new(
        texture_creator: &'static TextureCreator<WindowContext>,
        asset: ImageAsset,
        rect_idx: u32,
    ) -> Result<Sprite, TextureValueError> {
        let rect = asset.rect_for_idx(rect_idx);
        let texture: Texture<'static> = asset.surface.as_texture(texture_creator)?;
        Ok(Sprite { texture, rect })
    }

    pub fn render(&self, canvas: &mut WindowCanvas, center: Point) -> Result<(), String> {
        let rect = Rect::from_center(center, self.rect.width(), self.rect.height());
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
