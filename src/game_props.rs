pub const TILE_SIZE: u32 = 48;

pub const PHYSICS_SIMULATION_FPS: u32 = 60;
pub const PHYSICS_DELTA_TIME: f32 = 1.0 / PHYSICS_SIMULATION_FPS as f32;

pub const ANTIQUE_WHITE: (u8, u8, u8) = (0xfa, 0xeb, 0xd7);
pub const GRID_COLOR: (u8, u8, u8) = (128, 128, 128);
pub const PLAYER_HIT_TILE_COLOR: (u8, u8, u8) = (0x33, 0x33, 0xff);

pub const THRUST_ACCELERATION: f32 = 20.0;

pub const PLANETS_FRONT_LERP: f32 = 0.15;
pub const PLATFORM_LERP: f32 = 1.0;
