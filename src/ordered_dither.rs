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
        for (i, pixel) in copy.pixels_mut().enumerate() {
            pixel.0[0] = if pixel.0[0] > 1.0 - threshold_map.sample(i % width, i / width) {
                1.0
            } else {
                0.0
            }
        }
        copy.into()
    }
}
