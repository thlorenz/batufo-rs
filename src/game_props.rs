use serde_derive::Serialize;

#[derive(Serialize)]
pub struct GameColors {
    pub antique_white: (u8, u8, u8),
    pub grid_color: (u8, u8, u8),
    pub player_hit_tile_color: (u8, u8, u8),
}

#[derive(Serialize)]
pub struct GameProps {
    pub tile_size: u32,

    pub physics_simulation_fps: u32,
    #[serde(skip)] // calculated
    pub physics_delta_time: f32,
    pub thrust_acceleration: f32,

    pub planets_front_lerp: f32,
    pub platform_lerp: f32,

    pub colors: GameColors,
}

impl GameProps {
    pub fn default() -> Self {
        let tile_size = 48;

        let physics_simulation_fps: u32 = 60;
        let physics_delta_time: f32 = 1.0 / physics_simulation_fps as f32;

        let antique_white = (0xfa, 0xeb, 0xd7);
        let grid_color = (128, 128, 128);
        let player_hit_tile_color = (0x33, 0x33, 0xff);

        let thrust_acceleration: f32 = 20.0;

        let planets_front_lerp: f32 = 0.85;
        let platform_lerp: f32 = 1.0;

        let colors = GameColors {
            antique_white,
            grid_color,
            player_hit_tile_color,
        };

        GameProps {
            tile_size,

            physics_simulation_fps,
            physics_delta_time,

            thrust_acceleration,

            planets_front_lerp,
            platform_lerp,

            colors,
        }
    }

    pub fn to_toml(&self) {
        let s = toml::to_string(self).unwrap();
        eprintln!("toml: {:?}", s);
    }
}
