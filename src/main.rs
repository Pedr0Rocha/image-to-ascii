use core::panic;
use std::path::PathBuf;
use image::{GenericImageView, Pixel, Rgb};
use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, required = true)]
    image: PathBuf,

    #[arg(short = 'x', long, default_value_t = 80)]
    width: u32,

    #[arg(short = 'y', long, default_value_t = 80)]
    height: u32,
}

fn main() {
    let args = Args::parse();

    let image_result = image::open(args.image);

    let img = match image_result {
        Ok(img) => img.resize(args.width, args.height, image::imageops::FilterType::Gaussian).rotate270(),
        Err(error) => panic!("Could not open image: {:?}", error),
    };

    let (width, height) = img.dimensions();

    for x in 0..width {
        for y in 0..height {
            let pixel_rgb = img.get_pixel(x, y).to_rgb();
            let brightness = get_brightness(pixel_rgb);
            // println!("R={} G={} B={} = {}", pixel_rgb[0], pixel_rgb[1], pixel_rgb[2], brightness);

            print!("{}", brightness_to_ascii(brightness))
        }
        println!()
    }
    // print!("image dimensions: {:?}\n", img.dimensions());
}

fn get_brightness(pixel: Rgb<u8>) -> u16 {
    (pixel[0] as u16 + pixel[1] as u16 + pixel[2] as u16) / 3
}

fn brightness_to_ascii(brightness: u16) -> char {
    match brightness {
        0..=85 => '#',
        86..=120 => '%',
        121..=170 => '+',
        _ => '.',
    }
}
