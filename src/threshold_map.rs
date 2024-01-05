/// Represents the threshold map Bayer matrix used for dithering
pub struct ThresholdMap(pub(crate) Vec<Vec<f32>>);

impl ThresholdMap {

    /// Generates a new threshold map with a given level *N*, where the resulting matrix will be a square with 2 ^ (N + 1) rows and columns.
    ///
    /// This is an adaptation of an algorithm found [here](http://web.archive.org/web/20231225012832/https://bisqwit.iki.fi/story/howto/dither/jy/) 
    /// (See appendix 2). It produces an imperfect but "good enough" matrix for our purposes. For
    /// further information on threshold maps, see the Wikipedia article on [Ordered Dithering](https://en.wikipedia.org/wiki/Ordered_dithering).
    ///
    /// # Arguments
    ///
    /// * `level` - Level of Bayer matrix to generate
    ///
    pub fn level(level: u32) -> Self {
        let power = level + 1;
        let size = 1 << power;
        let mut matrix = vec![vec![0.0; size]; size];
        for (i, row) in matrix.iter_mut().enumerate() {
            for (j, cell) in row.iter_mut().enumerate() {
                // XOR x and y coordinate
                let a = (i ^ j) as u32;
                // Interleave bits of `a` with bits of y coordinate in reverse order
                let mut result: u64 = 0;
                let mut bit = 0;
                let mut mask = power as i32 - 1;
                loop {
                    if bit >= 2 * power {
                        break;
                    }
                    result |= (((j >> mask) & 1) << bit) as u64;
                    bit += 1;
                    result |= (((a >> mask) & 1) << bit) as u64;
                    bit += 1;
                    mask -= 1;
                }
                // Divide value to get a value between 0.0 and 1.0
                *cell = result as f32 / size.pow(2) as f32;
            }
        }

        Self(matrix)
    }

    /// Sample this threshold at the given x and y coordinates, wrapping around as necessary
    ///
    /// # Arguments
    ///
    /// * `x` - x coordinate to sample at
    /// * `y` - y coordinate to sample at
    ///
    pub fn sample(&self, x: usize, y: usize) -> f32 {
        let size = self.0.len();
        self.0[x % size][y % size]
    }
}
