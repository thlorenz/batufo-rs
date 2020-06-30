extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

use sdl2::surface::Surface;
use sdl2::rect::{Rect};
use std::fmt::Debug;
use sdl2::video::Window;

#[derive(Debug)]
struct ImageAsset {
    width: i32,
    height: i32,
    rows: i32,
    cols: i32,
    tiles: u32,
    item_width: i32,
    item_height: i32,
    path: &'static str,
}


impl ImageAsset {
    // TODO: support loading other image types than bmp
    // TODO: once we figure out how add surface to the asset itself via mut
    fn load(&self) -> Result<Surface<'static>, String> {
        Surface::load_bmp(&self.path)
    }

    fn rect(&self, row: u32, col: u32) -> Rect {
        let x: i32 = col as i32 * self.item_width;
        let y: i32 = row as i32 * self.item_height;
        Rect::new(x, y, self.item_width as u32, self.item_height as u32)
    }

    fn rect_for_idx(&self, idx: u32) -> Rect {
        let row = idx / self.rows as u32;
        let col = idx % self.rows as u32;
        self.rect(row, col)
    }

    fn new(
        width: i32,
        height: i32,
        rows: i32,
        cols: i32,
        path: &'static str) -> ImageAsset {
        ImageAsset {
            width,
            height,
            rows,
            cols,
            path,
            tiles: (rows * cols) as u32,
            item_width: width / cols,
            item_height: height / rows,
        }
    }
}


pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window: Window = video_subsystem.window("batufo-rs", 800, 600)
        .position(-600, 0)
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().accelerated().build().map_err(|e| e.to_string())?;

    let asset = ImageAsset::new(
        1024,
        1024,
        8,
        8,
        "assets/images/bg/floor-tiles.bmp");

    let surface: Surface = asset.load()
        .map_err(|err| format!("failed to load {:?} {}", asset, err))?;

    let texture_creator = canvas.texture_creator();
    let texture = texture_creator.create_texture_from_surface(&surface)
        .map_err(|e| e.to_string())?;

    let win = canvas.window();
    let size = win.size();
    let rows: i32 = (size.1 / asset.item_height as u32) as i32;
    let cols: i32 = (size.0 / asset.item_width as u32) as i32;

    {
        canvas.clear();
        let mut idx:u32 = 0;
        for row in 0..rows {
            for col in 0..cols {
                idx += 1;
                if idx >= asset.tiles { idx = 0; }
                let src_rect = asset.rect_for_idx(idx);
                let x: i32 = col * asset.item_width;
                let y: i32 = row * asset.item_height;
                let dst_rect = Rect::new(x, y, asset.item_width as u32, asset.item_height as u32);
                canvas.copy_ex(&texture, Some(src_rect), Some(dst_rect), 0.0, None, false, false)?;
            }
        }
        canvas.present();
    }

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                }
                _ => {}
            }
        }
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
