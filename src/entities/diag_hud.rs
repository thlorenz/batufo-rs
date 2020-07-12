use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::WindowCanvas;

use crate::data::diagnostics::Diagnostic;
use crate::entities::text::{FontBlend, Text};
use std::error::Error;

pub struct DiagHud<'a> {
    position: Point,
    background_color: Color,
    height: u32,

    stats_text: Text<'a>,
}

impl<'a> DiagHud<'a> {
    pub fn new(position: Point, stats_text: Text<'a>) -> DiagHud {
        let background_color = Color::RGBA(0, 0, 0, 0xaa);
        let height = 40;
        DiagHud {
            position,
            background_color,
            height,
            stats_text,
        }
    }

    pub fn render(
        &self,
        canvas: &mut WindowCanvas,
        diagnostics: &Diagnostic,
    ) -> Result<(), Box<dyn Error>> {
        let (width, _) = canvas.window().size();
        let rect = Rect::new(self.position.x, self.position.y, width, self.height);

        canvas.set_draw_color(self.background_color);
        canvas.fill_rect(rect)?;

        let stats: String = format!(
            "fps: {fps} (P:{poll:02} U:{calc:02} R:{rndr:02}) -> {tot:02}ms +{rem:02}ms",
            fps = diagnostics.frame_rate,
            poll = diagnostics.time_spent_polling_ms,
            calc = diagnostics.time_spent_udpating_ms,
            rndr = diagnostics.time_spent_rendering_ms,
            tot = diagnostics.time_spent_total_ms,
            rem = diagnostics.time_budget_left_ms
        );

        self.stats_text.render(
            canvas,
            Point::new(10, 10),
            &stats,
            Color::WHITE,
            FontBlend::Blended,
        )?;
        Ok(())
    }
}
