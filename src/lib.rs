// TODO: remove once we got things integrated
#![allow(dead_code)]
use std::error::Error;
use std::fmt;
use std::time::Duration;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use sdl2::video::{Window, WindowBuildError, WindowBuilder};
use sdl2::{IntegerOrSdlError, Sdl, VideoSubsystem};

use crate::arena::arena::Arena;
use crate::engine::assets::image_asset::{ImageAsset, ImageAssets};
use crate::game::Game;

mod arena;
mod engine;
mod entities;
mod game;
mod game_props;

#[derive(fmt::Debug)]
pub struct WindowSettings {
    pub x: i32,
    pub y: i32,
    pub title: &'static str,
    pub width: u32,
    pub height: u32,
    pub resizable: bool,
}

#[derive(fmt::Debug)]
pub struct Config {
    pub window_settings: WindowSettings,
}

pub fn start(config: &Config) -> Result<(), Box<dyn Error>> {
    println!("config {:?}", config);

    let sdl_context: Sdl = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window: Window = build_window(&video_subsystem, &config.window_settings)?;

    let mut canvas = build_canvas(window)?;
    let texture_creator = canvas.texture_creator();

    let image_assets = ImageAssets::new(&texture_creator)?;
    let floor_asset: &ImageAsset = image_assets
        .assets
        .get("floor-tiles")
        .expect("floor-tiles not loaded");

    println!("floor tiles ${:?}", floor_asset);

    // draw_buildings(&mut canvas, &floor_asset)?;

    let arena = Arena::for_level("face off")?;
    let game = Game::new(&arena, floor_asset)?;

    println!("starting event loop");
    start_event_loop(&sdl_context, game, &mut canvas);
    Ok(())
}

fn start_event_loop(sdl_context: &Sdl, game: Game, canvas: &mut WindowCanvas) {
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
        game.render(canvas).expect("FATAL: game render failed");
        ::std::thread::sleep(Duration::from_millis(16));
    }
}

// TODO: draw level here
fn draw_buildings(canvas: &mut WindowCanvas, asset: &ImageAsset) -> Result<(), Box<dyn Error>> {
    let win = canvas.window();
    let size = win.size();
    let rows: u32 = size.1 / asset.item_height + 1;
    let cols: u32 = size.0 / asset.item_width + 1;
    let texture_creator = canvas.texture_creator();
    let texture = asset.surface.as_texture(&texture_creator)?;

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
    window.into_canvas().build()
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
