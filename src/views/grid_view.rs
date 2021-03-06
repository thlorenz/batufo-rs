use ggez::graphics::{Color, Mesh, MeshBuilder};
use ggez::nalgebra as na;
use ggez::Context;
use ggez::{graphics, GameResult};

use ggez::nalgebra::Point2;

const LINE_WIDTH: f32 = 1.0;

pub struct GridView {
    mesh: Mesh,
}

impl GridView {
    pub fn new(
        ctx: &mut Context,
        ncols: u32,
        nrows: u32,
        tile_size: u32,
        grid_color: Color,
    ) -> GameResult<Self> {
        let mesh = build_mesh(ctx, ncols, nrows, tile_size, grid_color)?;
        Ok(GridView { mesh })
    }

    pub fn render<P>(&self, ctx: &mut Context, origin: P) -> GameResult
    where
        P: Into<Point2<f32>>,
    {
        let draw_args = (origin.into(), 0.0, graphics::WHITE);
        graphics::draw(ctx, &self.mesh, draw_args)
    }
}

fn build_mesh(
    ctx: &mut Context,
    ncols: u32,
    nrows: u32,
    tile_size: u32,
    grid_color: Color,
) -> GameResult<Mesh> {
    let max_x = (ncols * tile_size) as f32;
    let max_y = (nrows * tile_size) as f32;

    let mut mesh_builder = MeshBuilder::new();
    for row in 0..nrows + 1 {
        let y = (row * tile_size) as f32;
        mesh_builder.line(
            &[na::Point2::new(0.0, y), na::Point2::new(max_x, y)],
            LINE_WIDTH,
            grid_color,
        )?;
    }
    for col in 0..ncols + 1 {
        let x = (col * tile_size) as f32;
        mesh_builder.line(
            &[na::Point2::new(x, 0.0), na::Point2::new(x, max_y)],
            LINE_WIDTH,
            grid_color,
        )?;
    }

    let mesh = mesh_builder.build(ctx)?;
    Ok(mesh)
}
