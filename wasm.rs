use crate::assets;
use crate::constants::*;
use crate::engine::Engine;

static mut ENGINE: Option<Engine> = None;

#[export_name = "init"]
pub unsafe extern "C" fn init(t: f64) -> i32 {
    if ENGINE.is_some() {
        return RET_FAILURE;
    }

    ENGINE = Some(Engine::new());
    ENGINE.as_mut().unwrap().init(t)
}

#[export_name = "update"]
pub unsafe extern "C" fn update(t: f64) -> i32 {
    if ENGINE.is_none() {
        return RET_FAILURE;
    }

    ENGINE.as_mut().unwrap().update(t)
}

extern "C" {
    fn get_keydown() -> i32;
    fn create_texture(ptr: *const u8, width: u32, height: u32) -> i32;
    fn set_render_target(idx: i32);
    fn set_render_source(idx: i32);
    fn set_line_color(idx: i32);
    fn set_fill_color(idx: i32);
    fn set_color_mod(idx: i32);
    fn set_rect_size(sw: i32, sh: i32, dw: i32, dh: i32);
    fn render_copy(sx: i32, sy: i32, dx: i32, dy: i32);
    fn fill_rect(x: i32, y: i32, w: i32, h: i32);
    fn toggle_scale_factor();
    fn begin_path();
    fn close_path();
    fn move_to(x: i32, y: i32);
    fn line_to(x: i32, y: i32);
    fn stroke();
}

#[derive(Clone, Debug, Default)]
pub struct Backend {
    textures: Vec<i32>,
}

impl Backend {
    pub fn new() -> Self {
        Self { textures: vec![] }
    }

    pub fn init(&mut self) -> i32 {
        RET_SUCCESS
    }

    pub fn get_keydown(&self) -> i32 {
        unsafe { get_keydown() }
    }

    pub fn set_fill_color(&self, idx: i32) {
        unsafe { set_fill_color(idx) }
    }

    pub fn set_color_mod(&self, idx: i32) {
        unsafe { set_color_mod(idx) }
    }

    pub fn set_render_target(&self, idx: i32) {
        unsafe { set_render_target(idx) }
    }

    pub fn set_render_source(&self, idx: i32) {
        unsafe { set_render_source(idx) }
    }

    pub fn fill_rect(&self, x: i32, y: i32, w: i32, h: i32) {
        unsafe { fill_rect(x, y, w, h) }
    }

    pub fn set_rect_size(&self, sw: i32, sh: i32, dw: i32, dh: i32) {
        unsafe { set_rect_size(sw, sh, dw, dh) }
    }

    pub fn render_copy(&self, sx: i32, sy: i32, dx: i32, dy: i32) {
        unsafe { render_copy(sx, sy, dx, dy) }
    }

    pub fn load_texture(&mut self, name: &'static str) -> i32 {
        let mut idxd = assets::load(name);
        let hvec = idxd.split_off(idxd.len() - 4);
        let wvec = idxd.split_off(idxd.len() - 4);
        let height = u32::from_le_bytes(hvec.try_into().unwrap());
        let width = u32::from_le_bytes(wvec.try_into().unwrap());
        assert_eq!(idxd.len(), (width * height) as usize);
        let mut rgba = Vec::with_capacity(idxd.len() * 4);

        for i in idxd {
            let j = i as usize * 4;
            rgba.extend_from_slice(&PALETTE[j..j + 4]);
        }

        let idx = unsafe { create_texture(rgba.as_ptr(), width, height) };

        if idx != 0 {
            self.textures.push(idx);
        } else {
            panic!("aaaaaaaaa");
        }

        idx
    }
}
