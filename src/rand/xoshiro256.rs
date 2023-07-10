use cargo_snippet::snippet;

/// seed値を4つの初期状態値に分割するためのsplit mix 64
#[snippet("r3yohei_split_mix_64")]
struct SplitMix64 {
    s: u64,
}
#[snippet("r3yohei_split_mix_64")]
impl SplitMix64 {
    fn new(seed: u64) -> Self {
        Self {s: seed}
    }
    fn next_u64(&mut self) -> u64 {
        self.s = self.s.wrapping_add(0x9e3779b97f4a7c15);
        let mut z = self.s;
        z = (z ^ z >> 30).wrapping_mul(0xbf58476d1ce4e5b9);
        z = (z ^ z >> 27).wrapping_mul(0x94d049bb133111eb);
        z ^ z >> 31
    }
}

/// Xoshiro256による乱数生成器
#[snippet("r3yohei_Xoshiro256")]
#[snippet(include = "r3yohei_split_mix_64")]
struct Xoshiro256 {
    s: [u64; 4],
}
#[snippet("r3yohei_Xoshiro256")]
#[snippet(include = "r3yohei_split_mix_64")]
impl Xoshiro256 {
    fn new(seed: u64) -> Self {
        let mut split_mix_64 = SplitMix64::new(seed);
        let mut s = [0; 4];
        for si in &mut s {
            *si = split_mix_64.next_u64();
        }
        Self { s }
    }

    fn next_u64(&mut self) -> u64 {
        let result = (self.s[1].wrapping_mul(5)).rotate_left(7).wrapping_mul(9);
        let t = self.s[1] << 17;

        self.s[2] ^= self.s[0];
        self.s[3] ^= self.s[1];
        self.s[1] ^= self.s[2];
        self.s[0] ^= self.s[3];
        self.s[2] ^= t;
        self.s[3] = self.s[3].rotate_left(45);

        result
    }

    fn gen_usize(&mut self, lower: usize, upper: usize) -> usize {
        assert!(lower < upper);
        let count = upper - lower;
        (self.next_u64() % count as u64) as usize + lower
    }

    fn gen_i64(&mut self, lower: i64, upper: i64) -> i64 {
        assert!(lower < upper);
        let count = upper - lower;
        (self.next_u64() % count as u64) as i64 + lower
    }

    fn gen_f64(&mut self) -> f64 {
        const UPPER_MASK: u64 = 0x3ff0000000000000;
        const LOWER_MASK: u64 = 0xfffffffffffff;
        let result = UPPER_MASK | (self.next_u64() & LOWER_MASK);
        let result: f64 = unsafe { std::mem::transmute(result) };
        result - 1.0
    }

    fn gen_bool(&mut self, prob: f64) -> bool {
        self.gen_f64() < prob
    }
}