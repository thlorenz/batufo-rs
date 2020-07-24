use sdl2::image::LoadTexture;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;

use sdl2::rect::Rect;
use sdl2::render::{Texture, TextureCreator};
use sdl2::video::WindowContext;

struct ImageAssetConf {
    pub width: u32,
    pub height: u32,
    pub rows: u32,
    pub cols: u32,
    pub path: &'static str,
}

impl ImageAssetConf {
    fn new(width: u32, height: u32, rows: u32, cols: u32, path: &'static str) -> ImageAssetConf {
        ImageAssetConf {
            width,
            height,
            rows,
            cols,
            path,
        }
    }
}

pub struct ImageAsset<'a> {
    pub width: u32,
    pub height: u32,
    pub rows: u32,
    pub cols: u32,
    pub tiles: u32,
    pub item_width: u32,
    pub item_height: u32,
    pub texture: Texture<'a>,
    pub path: &'static str,
}

impl<'a> ImageAsset<'a> {
    fn new(
        asset: ImageAssetConf,
        texture_creator: &'a TextureCreator<WindowContext>,
    ) -> Result<Self, Box<dyn Error>> {
        let texture = texture_creator.load_texture(asset.path)?;
        Ok(ImageAsset {
            width: asset.width,
            height: asset.height,
            rows: asset.rows,
            cols: asset.cols,
            path: asset.path,
            tiles: (asset.rows * asset.cols),
            item_width: asset.width / asset.cols,
            item_height: asset.height / asset.rows,
            texture,
        })
    }

    pub fn rect(&self, row: u32, col: u32) -> Rect {
        let x: u32 = col * self.item_width;
        let y: u32 = row * self.item_height;
        Rect::new(x as i32, y as i32, self.item_width, self.item_height)
    }

    pub fn rect_for_idx(&self, idx: u32) -> Rect {
        let row = idx / self.rows;
        let col = idx % self.rows;
        self.rect(row, col)
    }
}

impl fmt::Debug for ImageAsset<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "\
ImageAsset {{
   tiles: {},
   rows: {}, cols: {},
   width: {} height: {},
   item_width: {}, item_height: {}
}}",
            self.tiles,
            self.rows,
            self.cols,
            self.width,
            self.height,
            self.item_width,
            self.item_height
        )
    }
}

fn load_all<'a>(
    confs: HashMap<&'static str, ImageAssetConf>,
    texture_creator: &'a TextureCreator<WindowContext>,
) -> Result<HashMap<&'static str, ImageAsset<'a>>, Box<dyn Error>> {
    let mut assets: HashMap<&'static str, ImageAsset> = HashMap::new();
    for (key, conf) in confs {
        let asset: ImageAsset = ImageAsset::new(conf, &texture_creator)?;
        assets.insert(key, asset);
    }
    Ok(assets)
}

pub struct ImageAssets<'a> {
    pub assets: HashMap<&'static str, ImageAsset<'a>>,
}

impl<'a> ImageAssets<'a> {
    pub(crate) fn new(
        texture_creator: &'a TextureCreator<WindowContext>,
    ) -> Result<ImageAssets<'a>, Box<dyn Error>> {
        // TODO: this can't live in the engine, at least the confs need to be created elsewhere
        let mut asset_confs: HashMap<&'static str, ImageAssetConf> = HashMap::new();
        asset_confs.insert(
            "floor-tiles",
            ImageAssetConf::new(384, 384, 8, 8, "assets/images/bg/floor-tiles.png"),
        );
        asset_confs.insert(
            "wall-metal",
            ImageAssetConf::new(48, 48, 1, 1, "assets/images/bg/wall-metal.png"),
        );
        let assets: HashMap<&str, ImageAsset> = load_all(asset_confs, texture_creator).unwrap();

        Ok(ImageAssets { assets })
    }
    pub fn get(&self, name: &'static str) -> Result<&ImageAsset, String> {
        self.assets
            .get(name)
            .ok_or(format!("asset {} not loaded", name))
    }
}
