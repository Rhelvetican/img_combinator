use image::{io::Reader as ImageReader, ColorType, DynamicImage, ImageError};
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

pub fn from_arr_to_tup<T: Copy>(arr: &[T; 4]) -> (T, T, T, T) {
    (arr[0], arr[1], arr[2], arr[3])
}
