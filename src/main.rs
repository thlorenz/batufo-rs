extern crate sdl2;

use batufo::{Config, WindowSettings};

pub fn main() {
    let window_settings = WindowSettings {
        x: -500,
        y: 0,
        title: "batufo",
        width: 500,
        height: 500,
        resizable: true,
    };
    let config = Config { window_settings };
    match batufo::start(&config) {
        Ok(_) => {}
        Err(err) => eprint!("fatal {:?}", err),
    }
}
