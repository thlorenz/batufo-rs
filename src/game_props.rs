use ggez::graphics::Color;
use ggez::GameResult;
use serde_derive::{Deserialize, Serialize};
use serde_hex::{SerHex, StrictPfx};
use std::fs::File;
use std::io::Read;

#[derive(Serialize, Deserialize)]
pub struct GameColors {
    #[serde(with = "SerHex::<StrictPfx>")]
    background: [u8; 3],
    #[serde(with = "SerHex::<StrictPfx>")]
    grid_color: [u8; 3],
    #[serde(with = "SerHex::<StrictPfx>")]
    player_hit_tile_color: [u8; 3],
}

impl GameColors {
    pub fn background(&self) -> Color {
        rgb_to_color(self.background)
    }
    pub fn grid_color(&self) -> Color {
        rgb_to_color(self.grid_color)
    }
    pub fn player_hit_tile_color(&self) -> Color {
        rgb_to_color(self.player_hit_tile_color)
    }
}

fn rgb_to_color(rgb: [u8; 3]) -> Color {
    Color::from_rgb(rgb[0], rgb[1], rgb[2])
}

#[derive(Serialize, Deserialize)]
pub struct GameCameras {
    pub planets_front_lerp: f32,
    pub platform_lerp: f32,
}

#[derive(Serialize, Deserialize)]
pub struct GamePhysics {
    pub simulation_fps: u32,
    #[serde(skip)] // calculated
    pub delta_time: f32,

    pub thrust_acceleration: f32,
}

#[derive(Serialize, Deserialize)]
pub struct GameRender {
    pub tile_size: u32,
}

#[derive(Serialize, Deserialize)]
pub struct GameProps {
    pub render: GameRender,
    pub physics: GamePhysics,
    pub cameras: GameCameras,
    pub colors: GameColors,
}

const GAME_PROPS_CONF_FILE: &str = "game_props.toml";
impl GameProps {
    #[allow(dead_code)]
    pub fn default() -> Self {
        let tile_size = 48;

        let physics_simulation_fps: u32 = 60;
        let physics_delta_time: f32 = 1.0 / physics_simulation_fps as f32;

        let background = [0xfa, 0xeb, 0xd7];
        let grid_color = [128, 128, 128];
        let player_hit_tile_color = [0x33, 0x33, 0xff];

        let thrust_acceleration: f32 = 20.0;

        let planets_front_lerp: f32 = 0.85;
        let platform_lerp: f32 = 1.0;

        let physics = GamePhysics {
            simulation_fps: physics_simulation_fps,
            delta_time: physics_delta_time,
            thrust_acceleration,
        };

        let colors = GameColors {
            background,
            grid_color,
            player_hit_tile_color,
        };

        let cameras = GameCameras {
            planets_front_lerp,
            platform_lerp,
        };

        let render = GameRender { tile_size };

        GameProps {
            render,
            physics,
            cameras,
            colors,
        }
    }

    pub fn load_conf() -> GameResult<GameProps> {
        let mut config_toml = String::new();
        let mut toml_file = File::open(format!("resources/{}", GAME_PROPS_CONF_FILE))?;
        toml_file.read_to_string(&mut config_toml)?;

        let mut game_props: GameProps = toml::from_str(&config_toml)?;
        game_props.physics.delta_time = 1.0 / game_props.physics.simulation_fps as f32;

        Ok(game_props)
    }
}
