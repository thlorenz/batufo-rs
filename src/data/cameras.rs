use sdl2::rect::{Point, Rect};

pub struct Cameras {
    pub platform: Rect,
}

impl Cameras {
    pub fn new() -> Cameras {
        Cameras {
            platform: Rect::new(0, 0, 0, 0),
        }
    }
    pub fn update(&mut self, player_coords: &Point, window_size: &(u32, u32)) {
        self.platform = viewport_centered_on(player_coords, window_size);
    }
}

fn viewport_centered_on(camera: &Point, window_size: &(u32, u32)) -> Rect {
    let hw: i32 = (window_size.0 / 2) as i32;
    let hh: i32 = (window_size.1 / 2) as i32;
    let x = camera.x - hw;
    let y = camera.y - hh;

    Rect::new(x, y, window_size.0, window_size.1)
}
