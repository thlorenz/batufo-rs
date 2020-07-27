use ggez::graphics::Color;

pub struct GameProps {
    pub tile_size: u32,

    pub physics_simulation_fps: u32,
    pub physics_delta_time: f32,

    pub antique_white: Color,
    pub grid_color: Color,
    pub player_hit_tile_color: Color,

    pub thrust_acceleration: f32,

    pub planets_front_lerp: f32,
    pub platform_lerp: f32,
}

impl GameProps {
    pub fn default() -> Self {
        let tile_size = 48;

        let physics_simulation_fps: u32 = 60;
        let physics_delta_time: f32 = 1.0 / physics_simulation_fps as f32;

        let antique_white: Color = (0xfa, 0xeb, 0xd7).into();
        let grid_color: Color = (128, 128, 128).into();
        let player_hit_tile_color: Color = (0x33, 0x33, 0xff).into();

        let thrust_acceleration: f32 = 20.0;

        let planets_front_lerp: f32 = 0.15;
        let platform_lerp: f32 = 1.0;

        GameProps {
            tile_size,

            physics_simulation_fps,
            physics_delta_time,

            antique_white,
            grid_color,
            player_hit_tile_color,

            thrust_acceleration,

            planets_front_lerp,
            platform_lerp,
        }
    }
}
