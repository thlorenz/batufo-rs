extern crate sdl2;

use batufo::{Config, WindowSettings};

pub fn main() {
    let window_settings = WindowSettings {
        x: -800,
        y: 0,
        title: "batufo",
        width: 800,
        height: 800,
        resizable: true,
    };
    let config = Config { window_settings };
    match batufo::start(&config) {
        Ok(_) => {}
        Err(err) => eprint!("fatal {:?}", err),
    }
}
