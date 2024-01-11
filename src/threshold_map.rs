/// Represents the threshold map Bayer matrix used for dithering
#[derive(Debug, PartialEq)]
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
        self.0[y % size][x % size]
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(0, ThresholdMap([
        [0.0, 3.0],
        [2.0, 1.0],
    ].map(|row: [f32;2] | row.map(|cell| cell / 4.0).into()).into()))]
    #[case(1, ThresholdMap([
        [ 0.0, 12.0,  3.0, 15.0],
        [ 8.0,  4.0, 11.0,  7.0],
        [ 2.0, 14.0,  1.0, 13.0],
        [10.0,  6.0,  9.0,  5.0],
    ].map(|row: [f32;4] | row.map(|cell| cell / 16.0).into()).into()))]
    #[case(2, ThresholdMap([
        [ 0.0, 48.0, 12.0, 60.0,  3.0, 51.0, 15.0, 63.0],
        [32.0, 16.0, 44.0, 28.0, 35.0, 19.0, 47.0, 31.0],
        [ 8.0, 56.0,  4.0, 52.0, 11.0, 59.0,  7.0, 55.0],
        [40.0, 24.0, 36.0, 20.0, 43.0, 27.0, 39.0, 23.0],
        [ 2.0, 50.0, 14.0, 62.0,  1.0, 49.0, 13.0, 61.0],
        [34.0, 18.0, 46.0, 30.0, 33.0, 17.0, 45.0, 29.0],
        [10.0, 58.0,  6.0, 54.0,  9.0, 57.0,  5.0, 53.0],
        [42.0, 26.0, 38.0, 22.0, 41.0, 25.0, 37.0, 21.0],
    ].map(|row: [f32;8] | row.map(|cell| cell / 64.0).into()).into()))]
    fn construct_level(#[case] level: u32, #[case] expected: ThresholdMap) {
        let map = ThresholdMap::level(level);
        assert_eq!(map, expected)
    }

    #[rstest]
    #[case(0, 0, 0.0)]
    #[case(2, 3, 36.0 / 64.0)]
    #[case(7, 7, 21.0 / 64.0)]
    #[case(8, 8, 0.0)]
    #[case(6, 8, 15.0 / 64.0)]
    fn sample(#[case] x: usize, #[case] y: usize, #[case] expected: f32) {
        let map = ThresholdMap([
            [ 0.0, 48.0, 12.0, 60.0,  3.0, 51.0, 15.0, 63.0],
            [32.0, 16.0, 44.0, 28.0, 35.0, 19.0, 47.0, 31.0],
            [ 8.0, 56.0,  4.0, 52.0, 11.0, 59.0,  7.0, 55.0],
            [40.0, 24.0, 36.0, 20.0, 43.0, 27.0, 39.0, 23.0],
            [ 2.0, 50.0, 14.0, 62.0,  1.0, 49.0, 13.0, 61.0],
            [34.0, 18.0, 46.0, 30.0, 33.0, 17.0, 45.0, 29.0],
            [10.0, 58.0,  6.0, 54.0,  9.0, 57.0,  5.0, 53.0],
            [42.0, 26.0, 38.0, 22.0, 41.0, 25.0, 37.0, 21.0],
        ].map(|row: [f32;8] | row.map(|cell| cell / 64.0).into()).into());
        assert_eq!(map.sample(x, y), expected)
    }
}
