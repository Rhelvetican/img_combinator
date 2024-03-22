use image::{io::Reader as ImageReader, ColorType, DynamicImage, ImageBuffer, ImageError, Rgba};
use std::io::{stdin, Error};

pub fn read_image(path: &str) -> DynamicImage {
    match ImageReader::open(path) {
        Ok(img) => match img.decode() {
            Ok(img) => img,
            Err(e) => img_err_handle(e),
        },
        Err(e) => {
            handle_io_error(e);
            DynamicImage::new(0, 0, ColorType::Rgb16)
        }
    }
}

fn img_err_handle(_: ImageError) -> DynamicImage {
    DynamicImage::new(0, 0, ColorType::Rgb16)
}

fn handle_io_error(err: Error) {
    panic!("Error: {}", err);
}

pub fn read_line(msg: &str) -> String {
    let mut input = String::new();
    println!("{}", msg);
    match stdin().read_line(&mut input) {
        Ok(_) => {}
        Err(e) => handle_io_error(e),
    };
    input.trim().to_string()
}

pub fn get_pixels(
    buffers: &Vec<ImageBuffer<Rgba<u16>, Vec<u16>>>,
    x: u32,
    y: u32,
) -> Vec<Rgba<u16>> {
    let mut pixels: Vec<Rgba<u16>> = Vec::new();
    for buffer in buffers {
        pixels.push(*buffer.get_pixel(x, y));
    }
    pixels
}

pub fn get_avg_pixel(pixels: Vec<Rgba<u16>>) -> Rgba<u16> {
    let mut avg_r = Vec::new();
    let mut avg_g = Vec::new();
    let mut avg_b = Vec::new();
    for pixel in pixels {
        avg_r.push(pixel[0]);
        avg_g.push(pixel[1]);
        avg_b.push(pixel[2]);
    }
    Rgba([
        avg_r.iter().sum::<u16>() / avg_r.len() as u16,
        avg_g.iter().sum::<u16>() / avg_g.len() as u16,
        avg_b.iter().sum::<u16>() / avg_b.len() as u16,
        255,
    ])
}
