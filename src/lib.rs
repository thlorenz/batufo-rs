#![feature(fn_traits)]
// TODO: remove once we got things integrated
#![allow(dead_code)]

use std::error::Error;
use std::fmt;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::WindowCanvas;
use sdl2::video::{Window, WindowBuildError, WindowBuilder};
use sdl2::{IntegerOrSdlError, Sdl, VideoSubsystem};

use crate::arena::arena::Arena;
use crate::engine::assets::image_asset::{ImageAsset, ImageAssets};
use crate::game::Game;
use crate::game_props::{RENDER_GPU_ACCELERATED, TIME_PER_FRAME_MS};
use crate::inputs::input::Input;

mod arena;
mod engine;
mod entities;
mod game;
mod game_props;
mod inputs;

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
    let floor_asset: &ImageAsset = image_assets.get("floor-tiles")?;
    let wall_asset: &ImageAsset = image_assets.get("wall-metal")?;

    // let arena = Arena::for_level("mini")?;
    let arena = Arena::for_level("face off")?;
    let mut game = Game::new(&arena, floor_asset, wall_asset)?;

    println!("starting event loop");
    start_event_loop(&sdl_context, &mut game, &mut canvas)?;
    Ok(())
}

// See: https://github.com/tsoding/something/blob/c3bf0545aaaf93aa8f88cb4254576587a0e0af74/src/something_main.cpp#L158
fn start_event_loop(
    sdl_context: &Sdl,
    game: &mut Game,
    canvas: &mut WindowCanvas,
) -> Result<(), String> {
    let mut input = Input::new();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut timer = sdl_context.timer()?;

    let mut prev_ticks = timer.ticks();
    'running: loop {
        // Ensure we don't work more than needed for frame rate
        // WARN: timer docs recommend other way to keep track of time
        let dt = timer.ticks() - prev_ticks;
        if dt < TIME_PER_FRAME_MS {
            timer.delay(TIME_PER_FRAME_MS - dt - 1);
        }
        let curr_ticks = timer.ticks();
        let dt = curr_ticks - prev_ticks;
        prev_ticks = curr_ticks;

        input.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::A),
                    ..
                } => input.left(),
                Event::KeyDown {
                    keycode: Some(Keycode::D),
                    ..
                } => input.right(),
                Event::KeyDown {
                    keycode: Some(Keycode::W),
                    ..
                } => input.up(),
                Event::KeyDown {
                    keycode: Some(Keycode::S),
                    ..
                } => input.down(),
                _ => {}
            }
        }

        game.update(dt, &input);
        game.render(canvas).expect("FATAL: game render failed");
    }

    Ok(())
}

fn build_canvas(window: Window) -> Result<WindowCanvas, IntegerOrSdlError> {
    if RENDER_GPU_ACCELERATED {
        window.into_canvas().accelerated().build()
    } else {
        window.into_canvas().software().build()
    }
}

fn build_window(
    video_subsystem: &VideoSubsystem,
    w: &WindowSettings,
) -> Result<Window, WindowBuildError> {
    let mut builder: WindowBuilder = video_subsystem.window(w.title, w.width, w.height);
    builder.position(w.x, w.y).opengl();

    if w.resizable {
        builder.resizable();
    }
    builder.build()
}
