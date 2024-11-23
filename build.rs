use snap::write;
use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;

include!("src/constants.rs");

fn pack_assets() {
    let asset_dir = env::var("ASSET_DIR").unwrap_or("assets".to_string());
    let out_dir = "src/.include";
    let _ = fs::create_dir(out_dir).unwrap_or(());
    let mut names = String::new();

    for entry in fs::read_dir(asset_dir).unwrap() {
        let entry = entry.unwrap();
        let in_path = entry.path();
        let name = entry.file_name().to_str().unwrap().to_owned();
        let in_data = fs::read(&in_path).unwrap();

        let out_data = if name.ends_with(".bmp") {
            get_image_data(&in_data)
        } else {
            in_data
        };

        let out_path = Path::new(&out_dir).join(&name);
        let mut wtr = File::create(out_path).unwrap();
        wtr.write_all(&out_data).unwrap();
        names.push_str(&format!("\"{}\",", &name));
    }

    let assets = format!("use crate::include_assets;\ninclude_assets!{{ {} }}\n", &names);
    fs::write("src/assets.rs", &assets).unwrap();
}

fn get_image_data(buf: &[u8]) -> Vec<u8> {
    assert_eq!(get_u32(buf, 0xe), 40);
    assert_eq!(get_u16(buf, 0x1c), 8);
    let width = get_u32(buf, 0x12);
    let height = get_u32(buf, 0x16);
    let data_size = get_u32(buf, 0x22) as usize;
    let w = width as usize;
    let h = height as usize;
    assert_eq!(w * h, data_size);
    let mut out = vec![0; data_size];
    let mut src: usize = buf.len();
    let mut dst: usize = 0;

    for _ in 0..h {
        src -= w;
        out[dst..dst + w].copy_from_slice(&buf[src..src + w]);
        dst += w;
    }

    out.extend(width.to_le_bytes());
    out.extend(height.to_le_bytes());
    assert_eq!(out.len(), data_size + 8);
    out
}

fn get_u16(buf: &[u8], offset: usize) -> u16 {
    u16::from_le_bytes(buf[offset..offset + 2].try_into().unwrap())
}

fn get_u32(buf: &[u8], offset: usize) -> u32 {
    u32::from_le_bytes(buf[offset..offset + 4].try_into().unwrap())
}

fn include_constants(tail: &str) -> String {
    let mut head = format!(
        "
        const UNKNOWN = {};
        const KEYMAP = {{
            'Backspace': {},
            'Tab': {},
            'Enter': {},
            'Escape': {},
            ' ': {},
            'ArrowRight': {},
            'ArrowLeft': {},
            'ArrowDown': {},
            'ArrowUp': {},
        }};
        const ORIG_WIDTH = {};
        const ORIG_HEIGHT = {};
        const PALETTE = {:?};
    ",
        KeyCode::UNKNOWN,
        KeyCode::BACKSPACE,
        KeyCode::TAB,
        KeyCode::RETURN,
        KeyCode::ESCAPE,
        KeyCode::SPACE,
        KeyCode::RIGHT,
        KeyCode::LEFT,
        KeyCode::DOWN,
        KeyCode::UP,
        ORIG_WIDTH,
        ORIG_HEIGHT,
        PALETTE
    );

    head.push_str(tail);
    head
}

fn minify_file(path: &Path, src: &str) {
    let minified = if path.to_str().unwrap().ends_with(".html") {
        minifier::html::minify(&src)
    } else {
        minifier::js::minify(&src).to_string()
    };
    fs::write(path, &minified).unwrap();
}

fn build_web_frontend() {
    let profile = env::var("PROFILE").expect("PROFILE not set");
    let target_dir = format!("target/wasm32-unknown-unknown/{}", &profile);
    let in_path = Path::new("src").join("main.js");
    let out_path = Path::new(&target_dir).join("main.js");
    let src = fs::read_to_string(in_path).unwrap();
    let main = include_constants(&src);
    minify_file(&out_path, &main);

    let other_files = ["index.html"];

    for f in other_files.iter() {
        let in_path = Path::new("src").join(f);
        let out_path = Path::new(&target_dir).join(f);
        let src = fs::read_to_string(in_path).unwrap();
        minify_file(&out_path, &src);
    }
}

fn main() {
    let target = env::var("TARGET").expect("TARGET not set");

    if target == "wasm32-unknown-unknown" {
        build_web_frontend();
    }

    pack_assets();
}
