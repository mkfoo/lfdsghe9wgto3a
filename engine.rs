use crate::constants::*;
use crate::cp437;
use crate::Backend;

#[derive(Copy, Clone, Debug)]
struct Clock {
    phase: f32,
    prev: f32,
    lag: f32,
}

impl Clock {
    fn new() -> Self {
        Self {
            phase: 0.0,
            prev: 0.0,
            lag: 0.0,
        }
    }

    fn advance(&mut self, t: f64) -> bool {
        let now = t as f32;
        let lag = now - self.prev;
        self.prev = now;

        if 0.0 < lag && lag < MAX_LAG {
            let incr = GAME_SPEED * lag;
            self.phase += incr;
            self.phase %= 1.0;
            self.lag = lag;
            true
        } else {
            false
        }
    }
}

#[derive(Clone, Debug)]
pub struct Engine {
    be: Backend,
    clock: Clock,
}

impl Engine {
    pub fn new() -> Self {
        Self {
            be: Backend::new(),
            clock: Clock::new(),
        }
    }

    pub fn init(&mut self, t: f64) -> i32 {
        self.clock.prev = t as f32;

        if self.be.init() != RET_SUCCESS {
            return RET_FAILURE;
        };

        self.be.load_texture("tileset.bmp");
        self.be.load_texture("font.bmp");
        RET_SUCCESS
    }

    pub fn update(&mut self, t_now: f64) -> i32 {
        if self.clock.advance(t_now) {
            let retval = self.update_state();
            self.render_all();
            retval
        } else {
            RET_SUCCESS
        }
    }

    fn update_state(&mut self) -> i32 {
        match self.be.get_keydown() {
            KeyCode::UP => {
                self.gs.c += 1;
                self.dirty = true;
            }
            KeyCode::DOWN => {
                self.gs.c -= 1;
                self.dirty = true;
            }
            KeyCode::RIGHT => {
                self.gs.c2 += 1;
                self.set_text_color(self.gs.c2);
                self.dirty = true;
            }
            KeyCode::LEFT => {
                self.gs.c2 -= 1;
                self.set_text_color(self.gs.c2);
                self.dirty = true;
            }
            _ => {}
        }

        RET_SUCCESS
    }

    fn set_text_color(&mut self, idx: i32) {
        self.be.set_render_target(2);
        self.be.set_color_mod(idx);
    }

    fn render_all(&mut self) {
        if self.dirty {
            self.be.set_render_target(0);
            self.be.set_fill_color(self.gs.c);
            self.be.fill_rect(0, 0, 0, 0);
            self.render_sprites();
            self.render_text(116, 80, cp437!('├' '┤' 'É' '└' '└' 'ó'));
            self.dirty = false;
        }
    }

    fn render_sprites(&mut self) {
        self.be.set_render_source(1);
        self.be.set_rect_size(16, 16, 16, 16);
        self.be.render_copy(16, 16, 16, 16);
    }

    fn render_text(&mut self, dx: i32, dy: i32, text: &[u8]) {
        self.be.set_render_source(2);
        self.be.set_rect_size(8, 8, 8, 8);
        let mut dx = dx;

        for ch in text.iter().map(|b| *b as i32) {
            let sx = ch % CHARS_PER_ROW * FONT_W;
            let sy = ch / CHARS_PER_ROW * FONT_H;
            self.be.render_copy(sx, sy, dx, dy);
            dx += FONT_W;
        }
    }
}
