use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{TextureCreator, TextureQuery, WindowCanvas};
use sdl2::ttf::{Font, FontStyle, Sdl2TtfContext};
use sdl2::video::WindowContext;
use std::error::Error;
use std::path::Path;

pub enum FontBlend {
    Solid,
    Blended,
}

pub struct Text<'a> {
    font: Font<'a, 'static>,
    texture_creator: &'a TextureCreator<WindowContext>,
}

impl<'a> Text<'a> {
    pub fn new(
        ttf_context: &'a Sdl2TtfContext,
        texture_creator: &'a TextureCreator<WindowContext>,
        font_path: &Path,
        font_size: u16,
        font_style: FontStyle,
    ) -> Result<Text<'a>, Box<dyn Error>> {
        let mut font = ttf_context.load_font(font_path, font_size)?;
        font.set_style(font_style);

        Ok(Text {
            font,
            texture_creator,
        })
    }

    pub fn render(
        &self,
        canvas: &mut WindowCanvas,
        bottom_left: Point,
        text: &str,
        color: Color,
        font_blend: FontBlend,
    ) -> Result<(), Box<dyn Error>> {
        let render = self.font.render(text);
        let surface = match font_blend {
            FontBlend::Solid => render.solid(color)?,
            FontBlend::Blended => render.blended(color)?,
        };
        let texture = self.texture_creator.create_texture_from_surface(&surface)?;
        let TextureQuery { width, height, .. } = texture.query();
        let rect = Rect::new(bottom_left.x, bottom_left.y, width, height);

        canvas.copy(&texture, None, Some(rect))?;
        Ok(())
    }
}
