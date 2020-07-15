use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::WindowCanvas;

pub struct GridView {
    ncols: u32,
    nrows: u32,
    tile_size: u32,
}

impl GridView {
    pub fn new(ncols: u32, nrows: u32, tile_size: u32) -> Self {
        GridView {
            ncols,
            nrows,
            tile_size,
        }
    }

    pub fn render(&self, canvas: &mut WindowCanvas, viewport: &Rect) -> Result<(), String> {
        canvas.set_draw_color(Color::GRAY);
        let origin = viewport.top_left();
        let max_x = (self.ncols * self.tile_size) as i32 - origin.x;
        let max_y = (self.nrows * self.tile_size) as i32 - origin.y;
        for row in 0..self.nrows + 1 {
            let y = (row * self.tile_size) as i32 - origin.y;
            canvas.draw_line(Point::new(0, y), Point::new(max_x, y))?;
        }
        for col in 0..self.ncols + 1 {
            let x = (col * self.tile_size) as i32 - origin.x;
            canvas.draw_line(Point::new(x, 0), Point::new(x, max_y))?;
        }
        Ok(())
    }
}
