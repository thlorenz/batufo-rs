extern crate sdl2;

use batufo;
use batufo::{Config, WindowSettings};

pub fn main() {
    let window_settings = WindowSettings {
        x: -1200,
        y: 0,
        title: "batufo",
        width: 500,
        height: 500,
        resizable: true,
    };
    let image_assets = batufo::create_assets();
    let config = Config {
        window_settings,
        image_assets,
    };
    batufo::start(&config).unwrap();
}
