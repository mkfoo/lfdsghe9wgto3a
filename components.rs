use crate::constants::*;

pub const TILE_SIZES: [(i32, i32); 10] = [
   (1 * BASE_W, 1 * BASE_H),
   (2 * BASE_W, 1 * BASE_H),
   (1 * BASE_W, 2 * BASE_H),
   (2 * BASE_W, 2 * BASE_H),
   (4 * BASE_W, 2 * BASE_H),
   (2 * BASE_W, 4 * BASE_H),
   (4 * BASE_W, 4 * BASE_H),
   (8 * BASE_W, 4 * BASE_H),
   (4 * BASE_W, 8 * BASE_H),
   (8 * BASE_W, 8 * BASE_H),
];

pub struct Flags(u16)    

impl Flags {
    pub const DIRTY: u16 = 1 << 0;
    pub const HIDDEN: u16 = 1 << 1;

    pub fn set(&mut self, f: u16) {
        self.0 |= f;
    }

    pub fn all_of(&self, f: u16) -> bool {
        self.0 & f == f
    }

    pub fn some_of(&self, f: u16) -> bool {
        self.0 & f != 0
    }

    pub fn none_of(&self, f: u16) -> bool {
        self.0 & f == 0
    }
}

pub struct Sprite {
    pub x: i16,
    pub y: i16,
    pub size: u16,
    pub tile: u16,
}

impl Sprite {
    pub fn new(x: i16, y: i16, size: u16, tile: u16) -> Self {
        Self {
            x,
            y,
            size,
            tile,
        }
    }
}

pub struct Animation {
    pub start: u16,
    pub len: u16,
    pub speed: f32,
    pub phase: f32,
}

impl Animation {
    pub fn new(start: u16, len: u16, speed: f32) -> Self {
        Self {
            start,
            len,
            speed,
            phase: 0.0,
        }
    }

    pub fn update(&mut self) {
        self.phase += self.speed;
        self.phase %= 1.0;
    }
}
