use std::fs::File;

extern crate batufo;
use batufo::game_props::GameProps;
use ggez::GameResult;
use std::io;

pub fn to_toml_file<W: io::Write>(game_props: &GameProps, file: &mut W) -> GameResult {
    let s = toml::to_vec(game_props)?;
    file.write_all(&s)?;
    Ok(())
}

pub fn main() -> ggez::GameResult {
    let props = GameProps::default();
    let mut toml_file = File::create("resources/game_props.toml")?;
    to_toml_file(&props, &mut toml_file)?;
    Ok(())
}
