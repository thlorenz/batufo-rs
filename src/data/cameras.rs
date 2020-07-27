use ggez::nalgebra::Point2;

pub struct Cameras {
    platform: Point2<f32>,
    planets_front: Point2<f32>,
    platform_lerp: f32,
    planets_front_lerp: f32,
}

impl Cameras {
    pub fn new(platform_lerp: f32, planets_front_lerp: f32) -> Cameras {
        Cameras {
            platform: Point2::new(0.0, 0.0),
            planets_front: Point2::new(0.0, 0.0),

            platform_lerp,
            planets_front_lerp,
        }
    }
    pub fn update(&mut self, dt: f32, player_coords: Point2<f32>, window_size: &(f64, f64)) {
        let center =
            origin_centered_on(player_coords, (window_size.0 as f32, window_size.1 as f32));
        self.follow(dt, center)
    }

    fn follow(&mut self, dt: f32, center: Point2<f32>) {
        let lerp = (2.5 * dt).min(self.platform_lerp);
        let dx = (center.x - self.platform.x) * lerp;
        let dy = (center.y - self.platform.y) * lerp;

        self.platform = Point2::new(self.platform.x + dx, self.platform.y + dy);
        self.planets_front = Point2::new(
            self.planets_front.x + dx * self.planets_front_lerp,
            self.planets_front.y + dy * self.planets_front_lerp,
        );
    }

    pub fn platform_origin(&self) -> Point2<f32> {
        Point2::new(-self.platform.x, -self.platform.y)
    }

    pub fn planets_front_origin(&self) -> Point2<f32> {
        Point2::new(-self.planets_front.x, -self.planets_front.y)
    }
}

fn origin_centered_on(center: Point2<f32>, window_size: (f32, f32)) -> Point2<f32> {
    let hw: f32 = window_size.0 / 2.0;
    let hh: f32 = window_size.1 / 2.0;
    let x = center.x - hw;
    let y = center.y - hh;

    Point2::new(x, y)
}
