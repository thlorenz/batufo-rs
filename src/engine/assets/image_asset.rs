use std::collections::HashMap;
use std::error::Error;
use std::fmt;

use sdl2::rect::Rect;
use sdl2::surface::Surface;

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

pub struct ImageAsset {
    pub width: u32,
    pub height: u32,
    pub rows: u32,
    pub cols: u32,
    pub tiles: u32,
    pub item_width: u32,
    pub item_height: u32,
    pub surface: Surface<'static>,
    // TODO: texture
    // pub texture: Texture<'static>,
    pub path: &'static str,
}

#[allow(dead_code)]
impl ImageAsset {
    fn new(asset: ImageAssetConf) -> Result<Self, Box<dyn Error>> {
        let surface = Surface::load_bmp(asset.path)?;

        Ok(ImageAsset {
            width: asset.width,
            height: asset.height,
            rows: asset.rows,
            cols: asset.cols,
            path: asset.path,
            tiles: (asset.rows * asset.cols),
            item_width: asset.width / asset.cols,
            item_height: asset.height / asset.rows,
            surface,
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

impl fmt::Debug for ImageAsset {
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

fn load_all(
    confs: HashMap<&'static str, ImageAssetConf>,
) -> Result<HashMap<&'static str, ImageAsset>, Box<dyn Error>> {
    let mut assets: HashMap<&'static str, ImageAsset> = HashMap::new();
    for (key, conf) in confs {
        let asset: ImageAsset = ImageAsset::new(conf)?;
        assets.insert(key, asset);
    }
    Ok(assets)
}

pub struct ImageAssets {
    pub assets: HashMap<&'static str, ImageAsset>,
}

impl ImageAssets {
    pub(crate) fn new() -> Result<ImageAssets, Box<dyn Error>> {
        let mut asset_confs: HashMap<&'static str, ImageAssetConf> = HashMap::new();
        asset_confs.insert(
            "floor-tiles",
            ImageAssetConf::new(1024, 1024, 8, 8, "assets/images/bg/floor-tiles.bmp"),
        );
        let assets: HashMap<&str, ImageAsset> = load_all(asset_confs).unwrap();

        Ok(ImageAssets { assets })
    }
}
