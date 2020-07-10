extern crate sdl2;

use batufo::{Config, WindowSettings};

pub fn main() {
    let window_settings = WindowSettings {
        x: -1400,
        y: 0,
        title: "batufo",
        width: 1400,
        height: 1400,
        resizable: true,
    };
    let config = Config { window_settings };
    match batufo::start(&config) {
        Ok(_) => {}
        Err(err) => eprint!("fatal {:?}", err),
    }
}
