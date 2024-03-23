mod utils;
use image::{open, ImageBuffer, Rgba};
use std::{
    fs::{read_dir, DirBuilder},
    vec::Vec,
};
use utils::*;

fn main() -> Result<(), String> {
    let mut buffers: Vec<ImageBuffer<Rgba<u16>, Vec<u16>>> = Vec::new();

    let files = match read_dir("input") {
        Ok(f) => f,
        Err(e) => return Err(format!("Error: {}", e)),
    };

    for file in files {
        let file = file.unwrap();
        let path = file.path();
        let img = match open(&path) {
            Ok(i) => i,
            Err(e) => {
                DirBuilder::new().recursive(true).create("input").unwrap();
                return Err(format!("Error: {}", e));
            }
        };

        let img = img.into_rgba16();
        buffers.push(img);
    }

    let (new_width, new_height) = match buffers.iter().map(|b| b.dimensions()).min() {
        Some(w) => w,
        None => return Err("No images found".to_string()),
    };

    let mut new_img: ImageBuffer<Rgba<u16>, Vec<u16>> = ImageBuffer::new(new_width, new_height);

    let mode = CombinationMode::from_string(read_line("Enter combination mode:"));

    for (x, y, pixel) in new_img.enumerate_pixels_mut() {
        *pixel = calc_pixel(get_pixels(&buffers, x, y), mode);
    }

    match new_img.save("output.png") {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Error: {}", e)),
    }
}
