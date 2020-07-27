use crate::game_props::PLATFORM_LERP;
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
    pub fn update(&mut self, dt: f32, player_coords: Point2<f32>, window_size: &(f64, f64)) {
        let center_rect =
            viewport_centered_on(player_coords, (window_size.0 as f32, window_size.1 as f32));
        let center = Point2::new(center_rect.x, center_rect.y);
        self.platform = follow(&center, &self.platform, dt, PLATFORM_LERP);
    }
}

fn follow(center: &Point2<f32>, camera: &Rect, dt: f32, min_lerp: f32) -> Rect {
    let lerp = (2.5 * dt).min(min_lerp);
    let dx = (center.x - camera.x) * lerp;
    let dy = (center.y - camera.y) * lerp;
    Rect::new(camera.x + dx, camera.y + dy, camera.w, camera.h)
}

fn viewport_centered_on(center: Point2<f32>, window_size: (f32, f32)) -> Rect {
    let hw: f32 = window_size.0 / 2.0;
    let hh: f32 = window_size.1 / 2.0;
    let x = center.x - hw;
    let y = center.y - hh;

    Rect::new(x, y, window_size.0, window_size.1)
}
