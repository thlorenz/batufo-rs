extern crate sdl2;

use batufo::{Config, WindowSettings};

pub fn main() {
    //let window_settings = small_window();
    let window_settings = large_window();
    let config = Config { window_settings };
    match batufo::start(&config) {
        Ok(_) => {}
        Err(err) => eprint!("fatal {:?}", err),
    }
}

#[allow(dead_code)]
fn large_window() -> WindowSettings {
    WindowSettings {
        x: -1200,
        y: 0,
        title: "batufo",
        width: 1200,
        height: 1200,
        resizable: true,
    }
}

#[allow(dead_code)]
fn small_window() -> WindowSettings {
    WindowSettings {
        x: -600,
        y: 0,
        title: "batufo",
        width: 600,
        height: 600,
        resizable: true,
    }
}
