use ggez::graphics::{Mesh, MeshBuilder};
use ggez::nalgebra as na;
use ggez::Context;
use ggez::{graphics, GameResult};

use crate::game_props::GRID_COLOR;

const LINE_WIDTH: f32 = 1.0;

pub struct GridView {
    ncols: u32,
    nrows: u32,
    tile_size: u32,
    mesh: Option<Mesh>,
}

impl GridView {
    pub fn new(ncols: u32, nrows: u32, tile_size: u32) -> Self {
        GridView {
            ncols,
            nrows,
            tile_size,
            mesh: None,
        }
    }

    pub fn render(&mut self, ctx: &mut Context) -> GameResult<()> {
        let draw_args = (na::Point2::new(0.0, 0.0), 0.0, graphics::WHITE);
        match &self.mesh {
            None => {
                let mesh = self.build_mesh(ctx)?;
                graphics::draw(ctx, &mesh, draw_args)?;
                self.mesh = Some(mesh);
            }
            Some(mesh) => graphics::draw(ctx, mesh, draw_args)?,
        };
        Ok(())
    }

    fn build_mesh(&self, ctx: &mut Context) -> GameResult<Mesh> {
        let max_x = (self.ncols * self.tile_size) as f32;
        let max_y = (self.nrows * self.tile_size) as f32;

        let mut mesh_builder = MeshBuilder::new();
        for row in 0..self.nrows + 1 {
            let y = (row * self.tile_size) as f32;
            mesh_builder.line(
                &[na::Point2::new(0.0, y), na::Point2::new(max_x, y)],
                LINE_WIDTH,
                GRID_COLOR.into(),
            )?;
        }
        for col in 0..self.ncols + 1 {
            let x = (col * self.tile_size) as f32;
            mesh_builder.line(
                &[na::Point2::new(x, 0.0), na::Point2::new(x, max_y)],
                LINE_WIDTH,
                GRID_COLOR.into(),
            )?;
        }

        let mesh = mesh_builder.build(ctx)?;
        Ok(mesh)
    }
}
