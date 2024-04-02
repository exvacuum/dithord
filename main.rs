use std::io::{self, BufRead, BufReader, Error, Read, Write};

use clap::Parser;
use dithord::{OrderedDither, ThresholdMap};
use image::io::Reader;

/// Bayer ordered dithering utility
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Input image file path
    pub input: String,

    /// Output image file path
    pub output: String,

    /// Threshold map level
    #[clap(short, long, default_value = "2")]
    pub level: u32,
}

pub fn main() {
    // Parse arguments
    let args = Args::parse();

    // Load input image

    let raw_reader: Result<Box<dyn BufRead + 'static>, Error> = if args.input == "-" {
        Ok(Box::new(BufReader::new(std::io::stdin())))
    } else {
        match std::fs::File::open(&args.input) {
            Ok(file) => Ok(Box::new(BufReader::new(file))),
            Err(err) => Err(err),
        }
    };

    let mut buffer = Vec::<u8>::new();
    raw_reader
        .expect("Failed to open image for reading.")
        .read_to_end(&mut buffer)
        .expect("Failed to read image data.");
    let reader = Reader::new(io::Cursor::new(buffer))
        .with_guessed_format()
        .expect("Failed to determine image format.");
    let mut image = reader.decode().expect("Failed to decode image.");

    // Generate threshold map
    let threshold_map = ThresholdMap::level(args.level);

    // Apply dithering
    image = image.ordered_dither(&threshold_map);

    // Save output image
    let image = image.to_rgba8();

    if args.output == "-" {
        let mut out_buffer = Vec::<u8>::new();
        image
            .write_to(
                &mut io::Cursor::new(&mut out_buffer),
                image::ImageFormat::Png,
            )
            .expect("Failed to encode output image.");
        let mut out = io::stdout();
        out.write_all(&out_buffer)
            .expect("Failed to write image bytes to stdout");
        out.flush().expect("Failed to flush stdout");
    } else {
        image
            .save(&args.output)
            .expect("Failed to save output image");
    }
}
