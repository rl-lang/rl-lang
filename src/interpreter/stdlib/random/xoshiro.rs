pub struct Xoshiro256 {
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
        for i in 0..4 {
            // golden ratio * 2^64
            x = x.wrapping_add(0x9e3779b97f4a7c15);
            // magic constants
            x = (x ^ (x >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
            x = (x ^ (x >> 27)).wrapping_mul(0x94d049bb133111eb);
            state[i] = x ^ (x >> 31);
        }

        Self { state }
    }

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

    // primitive functions
    pub fn generate_random_int(&mut self, min: i64, max: i64) -> i64 {
        let range = (max - min + 1) as u64;
        min + (self.next() % range) as i64
    }
}
