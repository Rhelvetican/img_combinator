use image::{ImageBuffer, Rgba};
use std::io::stdin;

#[derive(Clone, Copy)]
pub(crate) enum CombinationMode {
    Average,
    Maximum,
    Minimum,
}

impl CombinationMode {
    pub(super) fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "average" | "avg" => CombinationMode::Average,
            "maximum" | "max" => CombinationMode::Maximum,
            "minimum" | "min" => CombinationMode::Minimum,
            _ => CombinationMode::Average,
        }
    }
    pub(super) fn from_string(s: String) -> Self {
        CombinationMode::from_str(&s)
    }
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

pub fn calc_pixel(pixels: Vec<Rgba<u16>>, mode: CombinationMode) -> Rgba<u16> {
    match mode {
        CombinationMode::Average => {
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
                u16::MAX,
            ])
        }
        CombinationMode::Maximum => {
            let mut max_r = 0;
            let mut max_g = 0;
            let mut max_b = 0;
            for pixel in pixels {
                if pixel[0] > max_r {
                    max_r = pixel[0];
                }
                if pixel[1] > max_g {
                    max_g = pixel[1];
                }
                if pixel[2] > max_b {
                    max_b = pixel[2];
                }
            }
            Rgba([max_r, max_g, max_b, u16::MAX])
        }
        CombinationMode::Minimum => {
            let mut min_r = u16::MAX;
            let mut min_g = u16::MAX;
            let mut min_b = u16::MAX;
            for pixel in pixels {
                if pixel[0] < min_r {
                    min_r = pixel[0];
                }
                if pixel[1] < min_g {
                    min_g = pixel[1];
                }
                if pixel[2] < min_b {
                    min_b = pixel[2];
                }
            }
            Rgba([min_r, min_g, min_b, u16::MAX])
        }
    }
}

pub fn read_line(msg: &str) -> String {
    let mut input = String::new();
    println!("{}", msg);
    match stdin().read_line(&mut input) {
        Ok(_) => {}
        Err(e) => {
            println!("Error: {}", e);
            return "".to_string();
        }
    };
    input.trim().to_string()
}
