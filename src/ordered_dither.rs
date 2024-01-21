use std::time::Instant;

use image::DynamicImage;

use crate::ThresholdMap;

/// Trait which enables ordered dither operation
pub trait OrderedDither {
    /// Performs an ordered dither on a copy of self, returning the result.
    ///
    /// # Arguments
    ///
    /// * `threshold_map` - Threshold map to use for dithering
    ///
    fn ordered_dither(&self, threshold_map: ThresholdMap) -> Self;
}

impl OrderedDither for DynamicImage {
    fn ordered_dither(&self, threshold_map: ThresholdMap) -> Self {
        let width = self.width() as usize;
        // Convert image to luma float image for convenient comparison
        let mut copy = self.to_luma32f().clone();
        let start = Instant::now();
        copy.pixels_mut().enumerate().for_each(|(i, pixel)| {
            pixel.0[0] = test_pixel(&threshold_map, pixel.0[0], i % width, i / width) as u32 as f32;
        });
        println!("Dithered in {}", start.elapsed().as_millis());
        copy.into()
    }
}

/// Tests pixel luma against threshold map
fn test_pixel(map: &ThresholdMap, luma: f32,  x: usize, y: usize) -> bool {
    luma > map.sample(x, y)
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(1.0, 0, 0, true)]
    #[case(0.0, 0, 0, false)]
    #[case(0.25, 1, 1, false)]
    #[case(0.26, 1, 1, true)]
    fn pixel_test(#[case] luma: f32, #[case] x: usize, #[case] y: usize, #[case] expected: bool) {
        let map = ThresholdMap::level(3);
        assert_eq!(test_pixel(&map, luma, x, y), expected)
    }
}
