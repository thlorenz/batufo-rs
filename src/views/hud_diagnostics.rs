use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::WindowCanvas;

use crate::data::diagnostics::Diagnostic;
use crate::views::text::{FontBlend, Text};
use std::error::Error;

pub struct HudDiagnostics<'a> {
    position: Point,
    background_color: Color,
    height: u32,
    width_percent: f32,

    stats_text: Text<'a>,
}

impl<'a> HudDiagnostics<'a> {
    pub fn new(
        position: Point,
        height: u32,
        width_percent: f32,
        stats_text: Text<'a>,
    ) -> HudDiagnostics {
        let background_color = Color::RGBA(0, 0, 0, 0xcc);
        HudDiagnostics {
            position,
            background_color,
            height,
            width_percent,
            stats_text,
        }
    }

    pub fn render(
        &self,
        canvas: &mut WindowCanvas,
        diagnostics: &Diagnostic,
        window_size: &(u32, u32),
    ) -> Result<(), Box<dyn Error>> {
        let width = (window_size.0 as f32 * self.width_percent) as u32;
        let rect = Rect::new(self.position.x, self.position.y, width, self.height);

        canvas.set_draw_color(self.background_color);
        canvas.fill_rect(rect)?;

        let stats: String = format!(
            "FPS: {fps} (P:{poll:02} U:{calc:02} R:{rndr:02}) -> {tot:02}ms",
            fps = diagnostics.frame_rate,
            poll = diagnostics.time_spent_polling_ms,
            calc = diagnostics.time_spent_udpating_ms,
            rndr = diagnostics.time_spent_rendering_ms,
            tot = diagnostics.time_spent_total_ms,
        );

        self.stats_text.render(
            canvas,
            Point::new(10, 0),
            &stats,
            width,
            Color::WHITE,
            FontBlend::Blended,
        )?;
        Ok(())
    }
}
