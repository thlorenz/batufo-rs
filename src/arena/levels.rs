use std::collections::HashMap;
use std::error::Error;

use crate::arena::builtins::face_off::level_face_off;
use crate::arena::tilemap::Tilemap;

pub struct Level {
    pub name: &'static str,
    pub level_string: &'static str,
}

impl Level {
    fn new(name: &'static str, level_string: &'static str) -> Level {
        Level { name, level_string }
    }
    fn create_tilemap(&self, tile_size: f32) -> Result<Tilemap, Box<dyn Error>> {
        Tilemap::new(self.level_string, tile_size)
    }
}

pub struct Levels {
    levels: HashMap<&'static str, Level>,
}

impl Levels {
    pub fn new() {
        let mut levels: HashMap<&'static str, Level> = HashMap::new();
        let name = "face off";
        levels.insert(
            name,
            Level {
                name,
                level_string: level_face_off(),
            },
        );
    }
    pub fn is_valid_level(&self, level_name: &'static str) -> bool {
        self.levels.contains_key(level_name)
    }

    pub fn get_level(&self, level_name: &'static str) -> Option<&Level> {
        self.levels.get(level_name)
    }
}
