use std::cmp::max;
use std::error::Error;
use std::fmt;
use std::vec::Vec;

#[allow(dead_code)]
pub enum Tile {
    /* 0 */ OutOfBounds,
    /* 1 */ Empty,
    /* 2 */ Hole,
    /* 3 */ Wall,
    /* 4 */ Player,
    /* 5 */ Medkit,
    /* 6 */ Shield,
    /* 7 */ Bomb,
}

#[allow(dead_code)]
pub struct Tilemap {
    tiles: Vec<&'static Tile>,
    nrows: u32,
    ncols: u32,
}

#[allow(dead_code)]
fn get_bounds(row: &str) -> (usize, usize) {
    let start = row.find('=').unwrap_or(0);
    let end = row.rfind('=').unwrap_or(start);
    let end = if end == start { row.len() - 1 } else { end };
    (start, end)
}

impl Tilemap {
    #[allow(dead_code)]
    pub fn new(terrain: &str) -> Result<Self, Box<dyn Error>> {
        let lines: Vec<&str> = terrain
            .lines()
            .skip_while(|s| s.trim().is_empty())
            .take_while(|s| !s.trim().is_empty())
            .collect();

        let nrows = lines.len();
        let ncols = lines.iter().fold(0, |acc, s| max(acc, s.len()));
        let ntiles = nrows * ncols;

        let tiles: Vec<&Tile> = vec![&Tile::OutOfBounds; ntiles];
        #[allow(clippy::needless_range_loop)]
        for row in 0..nrows {
            let line = lines[row];
            let (start, end) = get_bounds(line);
            let _tile_row = nrows - row - 1;
            for _col in start..end {
                // TODO: read up on indexing strings
                // let char: char = line.get(col)?;
            }
        }

        println!("lines: ${:?}", lines);
        let _start = 0;

        Ok(Tilemap {
            tiles,
            nrows: nrows as u32,
            ncols: ncols as u32,
        })
    }
}

impl fmt::Debug for Tilemap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "\
Tilemap {{
  nrows: {nrows},
  ncols: {ncols}
}}",
            nrows = self.nrows,
            ncols = self.ncols
        )
    }
}

#[cfg(test)]
mod tilemap_tests {
    use super::*;

    #[test]
    fn valid_terrain() {
        let terrain = "


=======================
=         p           =
=                     =
=====           p     =
=     ====        =
=   d =  =        =
=     ====        =
=====                 ====
=   +   p       p        =
=                     ====
=======================

";
        let tilemap = Tilemap::new(terrain);
        print!("tilemap {:?}", tilemap);
    }
}
