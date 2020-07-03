extern crate sdl2;

use std::time::Duration;

use sdl2::{event::Event, keyboard::Keycode, rect::Rect, video::Window};

use crate::engine::assets::image_asset::ImageAsset;
use std::error::Error;

mod arena;
mod engine;

pub fn main() -> Result<(), Box<dyn Error>> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window: Window = video_subsystem
        .window("batufo", 1200, 600)
        .position(-1200, 0)
        .resizable()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window
        .into_canvas()
        .accelerated()
        .build()
        .map_err(|e| e.to_string())?;

    let asset = ImageAsset::new(1024, 1024, 8, 8, "assets/images/bg/floor-tiles.bmp")?;

    println!("loaded asset {:?}", asset);

    let texture_creator = canvas.texture_creator();
    let texture = texture_creator
        .create_texture_from_surface(&asset.surface)
        .map_err(|e| e.to_string())?;

    let win = canvas.window();
    let size = win.size();
    let rows: u32 = size.1 / asset.item_height + 1;
    let cols: u32 = size.0 / asset.item_width + 1;
    {
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
                let dst_rect = Rect::new(
                    x as i32,
                    y as i32,
                    asset.item_width as u32,
                    asset.item_height as u32,
                );
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
    }

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

    Ok(())
}
