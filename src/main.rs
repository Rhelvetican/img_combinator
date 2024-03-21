mod utils;
use image::{ImageBuffer, Rgba};
use std::vec::Vec;
use utils::*;

fn main() -> Result<(), String> {
    let img1 = read_image(&read_line("Enter the path to the first image: ")).to_rgba16();
    let img2 = read_image(&read_line("Enter the path to the second image: ")).to_rgba16();

    let (width1, height1) = img1.dimensions();
    let (width2, height2) = img2.dimensions();
    let new_width = if width1 < width2 { width1 } else { width2 };
    let new_height = if height1 < height2 { height1 } else { height2 };

    let mut new_img: ImageBuffer<Rgba<u16>, Vec<u16>> = ImageBuffer::new(new_width, new_height);

    // Generate an empty buffer
    for (x, y, pixel) in new_img.enumerate_pixels_mut() {
        *pixel = {
            let (r1, g1, b1, _) = from_arr_to_tup(&img1.get_pixel(x, y).0);
            let (r2, g2, b2, _) = from_arr_to_tup(&img2.get_pixel(x, y).0);
            let (r, g, b, a) = ((r1 + r2) / 2, (g1 + g2) / 2, (b1 + b2) / 2, 255);
            Rgba([r, g, b, a])
        };
    }

    match new_img.save("output.png") {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Error: {}", e)),
    }
}
