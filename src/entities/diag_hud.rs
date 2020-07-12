use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::WindowCanvas;

pub struct DiagHud {
    position: Point,
    background_color: Color,
    height: u32,
}

impl DiagHud {
    pub fn new(position: Point) -> DiagHud {
        let background_color = Color::RGBA(0, 0, 0, 0xaa);
        let height = 40;
        DiagHud {
            position,
            background_color,
            height,
        }
    }

    pub fn render(&self, canvas: &mut WindowCanvas) -> Result<(), String> {
        let (width, _) = canvas.window().size();
        let rect = Rect::new(self.position.x, self.position.y, width, self.height);

        canvas.set_draw_color(self.background_color);
        canvas.fill_rect(rect)?;

        Ok(())
    }
}
