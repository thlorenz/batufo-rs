pub struct Diagnostics {
    pub frame_rate: u32,
    pub time_spent_polling_ms: u32,
    pub time_spent_udpating_ms: u32,
    pub time_spent_rendering_ms: u32,
    pub time_spent_total_ms: u32,
    pub time_budget_left_ms: u32,
}

impl Diagnostics {
    pub fn new(
        time_budget_ms: u32,
        time_spent_polling_ms: u32,
        time_spent_udpating_ms: u32,
        time_spent_rendering_ms: u32,
        total_dt_ms: u32,
    ) -> Diagnostics {
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

        Diagnostics {
            frame_rate,
            time_spent_polling_ms,
            time_spent_udpating_ms,
            time_spent_rendering_ms,
            time_spent_total_ms,
            time_budget_left_ms,
        }
    }

    pub fn empty() -> Diagnostics {
        Diagnostics::new(0, 0, 0, 0, 0)
    }
}
