use ggez::graphics::Rect;
use ggez::nalgebra::Point2;

pub struct Cameras {
    pub platform: Rect,
}

impl Cameras {
    pub fn new() -> Cameras {
        Cameras {
            platform: Rect::new(0.0, 0.0, 0.0, 0.0),
        }
    }
    pub fn update(&mut self, player_coords: Point2<f32>, window_size: &(f32, f32)) {
        self.platform = viewport_centered_on(player_coords, window_size);
    }
}

fn viewport_centered_on(camera: Point2<f32>, window_size: &(f32, f32)) -> Rect {
    let hw: f32 = window_size.0 / 2.0;
    let hh: f32 = window_size.1 / 2.0;
    let x = camera.x - hw;
    let y = camera.y - hh;

    Rect::new(x, y, window_size.0, window_size.1)
}
