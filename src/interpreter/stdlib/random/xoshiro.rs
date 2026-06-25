//! Xoshiro256** pseudo-random number generator.
//!
//! A fast, high-quality 256-bit state PRNG. Seeded from [`SystemTime`] at construction.
//! The seed is spread across 4 `u64` state words using the finalizer from
//! `splitmix64` (golden ratio + two mixing steps).

pub struct Xoshiro256 {
    /// The 256-bit PRNG state as four `u64` words.
    state: [u64; 4],
}

impl Default for Xoshiro256 {
    fn default() -> Self {
        Self::new()
    }
}

impl Xoshiro256 {
    fn new() -> Self {
        let seed = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos() as u64)
            .unwrap_or(12345);

        // spread the seed into 4 u64
        let mut state = [0u64; 4];
        let mut x = seed;
        for i in &mut state {
            // golden ratio * 2^64
            x = x.wrapping_add(0x9e3779b97f4a7c15);
            // magic constants
            x = (x ^ (x >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
            x = (x ^ (x >> 27)).wrapping_mul(0x94d049bb133111eb);
            *i = x ^ (x >> 31);
        }

        Self { state }
    }

    /// Advances the state and returns the next raw `u64` output.
    fn next(&mut self) -> u64 {
        // compute result before scrambling the state
        let result = (self.state[0].wrapping_add(self.state[3]))
            .rotate_left(23)
            .wrapping_add(self.state[0]);

        // scramble the state
        let temp = self.state[1] << 17;
        self.state[2] ^= self.state[0];
        self.state[3] ^= self.state[1];
        self.state[1] ^= self.state[2];
        self.state[0] ^= self.state[3];
        self.state[2] ^= temp;
        self.state[3] = self.state[3].rotate_left(45);
        result
    }

    /// Returns a random `i64` in `[min, max]` inclusive.
    pub fn generate_random_int_range(&mut self, min: i64, max: i64) -> i64 {
        let range = (max as i128 - min as i128 + 1) as u128;
        let offset = (self.next() as u128) % range;
        (min as i128 + offset as i128) as i64
    }

    /// Returns a random `f64` in `[0.0, 1.0)`.
    pub fn generate_random_float(&mut self) -> f64 {
        self.next() as f64 / u64::MAX as f64
    }

    /// Returns a random `f64` in `[min, max)`.
    pub fn generate_random_float_range(&mut self, min: f64, max: f64) -> f64 {
        min + self.generate_random_float() * (max - min)
    }

    /// Returns `true` with probability `weight` (clamped to `[0.0, 1.0]`).
    pub fn generate_random_bool(&mut self, weight: f64) -> bool {
        self.generate_random_float() < weight.clamp(0.0, 1.0)
    }
}
