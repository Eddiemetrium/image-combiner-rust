mod args;
use args::Args;
use image::{io::Reader, DynamicImage, ImageFormat, };
use std::{io::BufReader, fs::File};

enum ImageDataErrors {
    DifferentImageFormats,
}
fn main() -> Result<(), ImageDataErrors > {
let args = Args::new();
let (image_1, image_format_1) = find_image_path(args.image_1);
let (image_2, image_format_2) = find_image_path(args.image_2);
 
 if image_format_1 != image_format_2 {
return Err(ImageDataErrors::DifferentImageFormats);
}
Ok(())
}

fn find_image_path(path: String) -> (image::DynamicImage, image::ImageFormat) {
    let image_reader = Reader::open(path).unwrap();
    let image_format = image_reader.format().unwrap();
    let image = image_reader.decode().unwrap();
    (image, image_format)


}

fn get_smallest_dimension((dim_1: (u32,u32), dim_2:(u32,u32))) ->(u32,u32) {
let pix_1 = dim_1.0 * dim_1.1;
let pix_2 = dim_2.0 * dim_2.1;
return if pix_1 < pix_2 {dim_1} else {dim_2};
}
