pub const TILE_SIZE: u32 = 24 * 2;
pub const RENDER_GRID: bool = true;

// Turning GPU acceleration off for now as it causes the Mac to freeze
// for up to 10secs at times, see:
// https://mrmacintosh.com/2020-002-update-causes-some-macs-to-freeze-when-using-hw-accelerated-video/
// Experience is the same, except that CPU usage is a lot larger of course.
pub const RENDER_GPU_ACCELERATED: bool = true;

pub const FRAME_RATE: u32 = 60;
pub const TIME_PER_FRAME_MS: u32 = 1000 / FRAME_RATE;
pub const MIN_TIME_PER_FRAME_MS: u32 = (TIME_PER_FRAME_MS as f32 * 0.9) as u32;

// Colors
pub const ANTIQUE_WHITE: (u8, u8, u8) = (0xfa, 0xeb, 0xd7);
pub const AMBER_ACCENT: (u8, u8, u8) = (0xFF, 0xE5, 0x7F);

// Debug
pub const DIAGNOSTICS_COUNT: usize = (FRAME_RATE / 5) as usize;

pub const HUD_DIAGNOSTICS_HEIGHT: u32 = 20;
