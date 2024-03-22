mod utils;
use image::{ImageBuffer, Rgba};
use std::vec::Vec;
use utils::*;

fn main() -> Result<(), String> {
    let mut buffers: Vec<ImageBuffer<Rgba<u16>, Vec<u16>>> = Vec::new();

    loop {
        match read_line("Enter the path of the image: ").as_str() {
            "exit" => break,
            path => buffers.push(read_image(path).to_rgba16()),
        };
    }

    let (new_width, new_height) = match buffers.iter().map(|b| b.dimensions()).min() {
        Some(w) => w,
        None => return Err("No images found".to_string()),
    };

    let mut new_img: ImageBuffer<Rgba<u16>, Vec<u16>> = ImageBuffer::new(new_width, new_height);

    for (x, y, pixel) in new_img.enumerate_pixels_mut() {
        *pixel = get_avg_pixel(get_pixels(&buffers, x, y));
    }

    match new_img.save("output.png") {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Error: {}", e)),
    }
}
