pub const BASE_H: i32 = 8;
pub const BASE_W: i32 = 8;
pub const CHARS_PER_ROW: i32 = 16;
pub const FONT_H: i32 = 8;
pub const FONT_W: i32 = 8;
pub const GAME_SPEED: f32 = 0.0012;
pub const MAX_LAG: f32 = 800.0;
pub const ORIG_HEIGHT: i32 = 192;
pub const ORIG_WIDTH: i32 = 256;
pub const RET_FAILURE: i32 = 1;
pub const RET_QUIT: i32 = -1;
pub const RET_SUCCESS: i32 = 0;

pub struct KeyCode;

impl KeyCode {
    pub const UNKNOWN: i32 = 0;
    pub const BACKSPACE: i32 = 8;
    pub const TAB: i32 = 9;
    pub const RETURN: i32 = 13;
    pub const ESCAPE: i32 = 27;
    pub const SPACE: i32 = 32;
    pub const RIGHT: i32 = 1073741903;
    pub const LEFT: i32 = 1073741904;
    pub const DOWN: i32 = 1073741905;
    pub const UP: i32 = 1073741906;
}

pub const PALETTE: [u8; 64] = [
    0x00, 0x00, 0x00, 0x00, // TRANSPARENT
    0x00, 0x00, 0x00, 0xff, // BLACK
    0x3e, 0xb8, 0x49, 0xff, // MEDIUM_GREEN
    0x74, 0xd0, 0x7d, 0xff, // LIGHT_GREEN
    0x59, 0x55, 0xe0, 0xff, // DARK_BLUE
    0x80, 0x76, 0xf1, 0xff, // LIGHT_BLUE
    0xb9, 0x5e, 0x51, 0xff, // DARK_RED
    0x65, 0xdb, 0xef, 0xff, // CYAN
    0xdb, 0x65, 0x59, 0xff, // MEDIUM_RED
    0xff, 0x89, 0x7d, 0xff, // LIGHT_RED
    0xcc, 0xc3, 0x5e, 0xff, // DARK_YELLOW
    0xde, 0xd0, 0x87, 0xff, // LIGHT_YELLOW
    0x3a, 0xa2, 0x41, 0xff, // DARK_GREEN
    0xb7, 0x66, 0xb5, 0xff, // MAGENTA
    0xcc, 0xcc, 0xcc, 0xff, // GRAY
    0xff, 0xff, 0xff, 0xff, // WHITE
];

