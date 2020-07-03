use std::collections::HashMap;
use std::error::Error;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use sdl2::video::{Window, WindowBuildError, WindowBuilder};
use sdl2::{IntegerOrSdlError, Sdl, VideoSubsystem};

use crate::engine::assets::image_asset::ImageAsset;
use std::time::Duration;

mod arena;
mod engine;

pub struct WindowSettings {
    pub x: i32,
    pub y: i32,
    pub title: &'static str,
    pub width: u32,
    pub height: u32,
    pub resizable: bool,
}

pub struct Config {
    pub window_settings: WindowSettings,
    pub image_assets: HashMap<&'static str, ImageAsset>,
}

pub fn start(config: &Config) -> Result<(), Box<dyn Error>> {
    let sdl_context: Sdl = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    load_assets(&config.image_assets)?;

    let window: Window = build_window(&video_subsystem, &config.window_settings)?;
    let canvas = build_canvas(window)?;

    // TODO: how to propagate error here if this is None?
    let floor_tiles = config.image_assets.get("floor-tiles").unwrap();
    draw_buildings(canvas, &floor_tiles)?;

    start_event_loop(&sdl_context);
    Ok(())
}

pub fn create_assets() -> HashMap<&'static str, ImageAsset> {
    let mut map: HashMap<&'static str, ImageAsset> = HashMap::new();
    map.insert(
        "floor-tiles",
        ImageAsset::new(1024, 1024, 8, 8, "assets/images/bg/floor-tiles.bmp"),
    );
    map
}

fn start_event_loop(sdl_context: &Sdl) {
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'running;
                }
                _ => {}
            }
        }
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

fn draw_buildings(mut canvas: WindowCanvas, asset: &ImageAsset) -> Result<(), Box<dyn Error>> {
    let win = canvas.window();
    let size = win.size();
    let rows: u32 = size.1 / asset.item_height + 1;
    let cols: u32 = size.0 / asset.item_width + 1;

    let texture = asset.texture.as_ref().unwrap();

    canvas.clear();

    let mut idx: u32 = 0;
    for row in 0..rows {
        for col in 0..cols {
            idx += 1;
            if idx >= asset.tiles {
                idx = 0;
            }
            let src_rect = asset.rect_for_idx(idx);
            let x = col * asset.item_width;
            let y = row * asset.item_height;
            let dst_rect = Rect::new(x as i32, y as i32, asset.item_width, asset.item_height);
            canvas.copy_ex(
                &texture,
                Some(src_rect),
                Some(dst_rect),
                0.0,
                None,
                false,
                false,
            )?;
        }
    }

    canvas.present();
    Ok(())
}

fn build_canvas(window: Window) -> Result<WindowCanvas, IntegerOrSdlError> {
    window.into_canvas().accelerated().build()
}

fn build_window(
    video_subsystem: &VideoSubsystem,
    w: &WindowSettings,
) -> Result<Window, WindowBuildError> {
    let mut builder: WindowBuilder = video_subsystem.window(w.title, w.width, w.height);
    builder.position(w.x, w.y);

    if w.resizable {
        builder.resizable();
    }
    builder.build()
}

fn load_assets(assets: &HashMap<&'static str, ImageAsset>) -> Result<(), Box<dyn Error>> {
    for _asset in assets.values() {
        // TODO: how can we mutate a value inside a map?
        // asset.load()?;
    }
    Ok(())
}
