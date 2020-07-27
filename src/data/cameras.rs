use ggez::graphics::Rect;
use ggez::nalgebra::Point2;

pub struct Cameras {
    pub platform: Rect,
    pub planets_front: Rect,
    platform_lerp: f32,
    planets_front_lerp: f32,
}

impl Cameras {
    pub fn new(platform_lerp: f32, planets_front_lerp: f32) -> Cameras {
        Cameras {
            platform: Rect::new(0.0, 0.0, 0.0, 0.0),
            planets_front: Rect::new(0.0, 0.0, 0.0, 0.0),

            platform_lerp,
            planets_front_lerp,
        }
    }
    pub fn update(&mut self, dt: f32, player_coords: Point2<f32>, window_size: &(f64, f64)) {
        let center_rect =
            viewport_centered_on(player_coords, (window_size.0 as f32, window_size.1 as f32));

        let center = Point2::new(center_rect.x, center_rect.y);
        self.follow(dt, center)
    }

    fn follow(self: &mut Self, dt: f32, center: Point2<f32>) {
        let lerp = (2.5 * dt).min(self.platform_lerp);
        let dx = (center.x - self.platform.x) * lerp;
        let dy = (center.y - self.platform.y) * lerp;

        self.platform = Rect::new(
            self.platform.x + dx,
            self.platform.y + dy,
            self.platform.w,
            self.platform.h,
        );
        self.planets_front = Rect::new(
            self.planets_front.x + dx * self.planets_front_lerp,
            self.planets_front.y + dy * self.planets_front_lerp,
            self.planets_front.w,
            self.planets_front.h,
        );
    }
}

fn viewport_centered_on(center: Point2<f32>, window_size: (f32, f32)) -> Rect {
    let hw: f32 = window_size.0 / 2.0;
    let hh: f32 = window_size.1 / 2.0;
    let x = center.x - hw;
    let y = center.y - hh;

    Rect::new(x, y, window_size.0, window_size.1)
}
