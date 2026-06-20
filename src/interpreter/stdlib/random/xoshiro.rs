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
            x = x.wrapping_add(0x9e3779b97f4a7c15);
            x = (x ^ (x >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
            x = (x ^ (x >> 27)).wrapping_mul(0x94d049bb133111eb);
            state[i] = x ^ (x >> 31);
        }

        Self { state }
    }
}
