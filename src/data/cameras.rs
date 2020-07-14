use sdl2::rect::Point;

pub struct Cameras {
    pub platform: Point,
}

impl Cameras {
    pub fn new() -> Cameras {
        Cameras {
            platform: Point::new(0, 0),
        }
    }
    pub fn update(&mut self, player_coords: &Point, window_size: &(u32, u32)) {
        let center_screen = Point::new((window_size.0 / 2) as i32, (window_size.1 / 2) as i32);
        let centered = Point::new(
            player_coords.x - center_screen.x,
            player_coords.y - center_screen.y,
        );

        self.platform = centered;
    }
}
