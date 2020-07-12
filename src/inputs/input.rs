#[rustfmt::skip]
mod input_flags {
    /* 0000 0000 */ pub const NONE: u8 = 0x0;
    /* 0000 0001 */ pub const LEFT: u8 = 0x1;
    /* 0000 0010 */ pub const RIGHT: u8 = 0x2;
    /* 0000 0100 */ pub const UP: u8 = 0x4;
    /* 0000 1000 */ pub const DOWN: u8 = 0x8;
    /* 0001 0000 */ pub const FIRE: u8 = 0x10;
}

pub struct Input {
    flags: u8,
}

impl Input {
    pub fn new() -> Input {
        Input {
            flags: input_flags::NONE,
        }
    }
    pub fn clear(&mut self) {
        self.flags = input_flags::NONE
    }
    pub fn left(&mut self) {
        self.add(input_flags::LEFT)
    }
    pub fn right(&mut self) {
        self.add(input_flags::RIGHT)
    }
    pub fn up(&mut self) {
        self.add(input_flags::UP)
    }
    pub fn down(&mut self) {
        self.add(input_flags::DOWN)
    }
    pub fn fire(&mut self) {
        self.add(input_flags::FIRE)
    }

    pub fn has_left(&self) -> bool {
        self.has(input_flags::LEFT)
    }
    pub fn has_right(&self) -> bool {
        self.has(input_flags::RIGHT)
    }
    pub fn has_up(&self) -> bool {
        self.has(input_flags::UP)
    }
    pub fn has_down(&self) -> bool {
        self.has(input_flags::DOWN)
    }
    pub fn has_fire(&self) -> bool {
        self.has(input_flags::FIRE)
    }

    fn add(&mut self, flag: u8) {
        self.flags = self.flags | flag;
    }

    fn has(&self, flag: u8) -> bool {
        flag & self.flags != 0
    }
}
