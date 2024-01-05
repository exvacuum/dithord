# dithord: Bayer ordered dithering crate

[![Crates.io Version](https://img.shields.io/crates/v/dithord)](https://crates.io/crates/dithord)

This crate provides functionality which allows for monochromatic ordered dithering using a Bayer threshold matrix. This method of dithering is less accurate than others but is fast and visually appealing.

Currently only monochromatic dithering is supported, but support for arbitrary palettes could be considered as a feature.

## Add to Project
```sh
cargo add dithord
```

## Usage

This crate provides the `OrderedDither` trait which is implemented for `image::DynamicImage`, along with a `ThresholdMap` struct for generating and storing the Bayer matrix.

### Example
```rs
use dithord::{ThresholdMap, OrderedDither};
use image::{io::Reader};

fn main() {
    let mut image = Reader::open("example.png").unwrap().decode().unwrap();

    // Generate level 2 (8x8) threshold map
    let map = threshold_map::ThresholdMap::level(2);
    // Apply dither
    image = image.ordered_dither(map);
}
```
