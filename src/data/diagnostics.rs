use crate::game_props::DIAGNOSTICS_COUNT;

#[derive(Clone, Copy)]
pub struct Diagnostic {
    pub frame_rate: u32,
    pub time_spent_polling_ms: u32,
    pub time_spent_udpating_ms: u32,
    pub time_spent_rendering_ms: u32,
    pub time_spent_total_ms: u32,
    pub time_budget_left_ms: u32,
}

impl Diagnostic {
    pub fn new(
        time_budget_ms: u32,
        time_spent_polling_ms: u32,
        time_spent_udpating_ms: u32,
        time_spent_rendering_ms: u32,
        total_dt_ms: u32,
    ) -> Diagnostic {
        let time_spent_total_ms =
            time_spent_polling_ms + time_spent_udpating_ms + time_spent_rendering_ms;
        let time_budget_left_ms = if time_budget_ms < time_spent_total_ms {
            0
        } else {
            time_budget_ms - time_spent_total_ms
        };

        let frame_rate = if total_dt_ms <= 0 {
            0
        } else {
            1000 / total_dt_ms
        };
        Diagnostic {
            frame_rate,
            time_spent_polling_ms,
            time_spent_udpating_ms,
            time_spent_rendering_ms,
            time_spent_total_ms,
            time_budget_left_ms,
        }
    }

    pub fn empty() -> Diagnostic {
        Diagnostic::new(0, 0, 0, 0, 0)
    }
}

pub struct Diagnostics {
    idx: usize,
    diagnostics: [Diagnostic; DIAGNOSTICS_COUNT],
    current: Diagnostic,
}

impl Diagnostics {
    pub fn new() -> Diagnostics {
        Diagnostics {
            idx: 0,
            diagnostics: [Diagnostic::empty(); DIAGNOSTICS_COUNT],
            current: Diagnostic::empty(),
        }
    }
    pub fn update(&mut self, diagnostic: Diagnostic) {
        self.diagnostics[self.idx] = diagnostic;
        self.idx = self.idx + 1;
        if self.idx == self.diagnostics.len() {
            self.idx = 0;
            self.current = self.aggregate()
        }
    }

    pub fn current(&self) -> Diagnostic {
        self.current
    }

    fn aggregate(&self) -> Diagnostic {
        let len = self.diagnostics.len() as u32;
        let sum: Diagnostic = self
            .diagnostics
            .iter()
            .fold(Diagnostic::empty(), |mut acc, &d| {
                acc.frame_rate = acc.frame_rate + d.frame_rate;
                acc.time_spent_polling_ms = acc.time_spent_polling_ms + d.time_spent_polling_ms;
                acc.time_spent_udpating_ms = acc.time_spent_udpating_ms + d.time_spent_udpating_ms;
                acc.time_spent_rendering_ms =
                    acc.time_spent_rendering_ms + d.time_spent_rendering_ms;
                acc.time_spent_total_ms = acc.time_spent_total_ms + d.time_spent_total_ms;
                acc.time_budget_left_ms = acc.time_budget_left_ms + d.time_budget_left_ms;
                acc
            });

        Diagnostic {
            frame_rate: sum.frame_rate / len,
            time_spent_polling_ms: sum.time_spent_polling_ms / len,
            time_spent_udpating_ms: sum.time_spent_udpating_ms / len,
            time_spent_rendering_ms: sum.time_spent_rendering_ms / len,
            time_spent_total_ms: sum.time_spent_total_ms / len,
            time_budget_left_ms: sum.time_budget_left_ms / len,
        }
    }
}
