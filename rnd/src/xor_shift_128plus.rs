use crate::seed::black_hole;
use crate::split_mix64::SplitMix64;

pub struct XorShift128Plus {
    x0: i64,
    x1: i64,
}

impl XorShift128Plus {
    #[inline]
    pub fn next_long(&mut self) -> i64 {
        let s0 = self.x1;
        let mut s1 = self.x0;
        let s = s1.wrapping_add(s0);
        s1 ^= s1 << 23;
        self.x1 = s1 ^ s0 ^ (s1 as u64 >> 18) as i64 ^ (s0 as u64 >> 5) as i64;
        self.x0 = s0;
        s
    }

    #[inline]
    pub fn new() -> Self {
        let mut rng = SplitMix64::new();
        XorShift128Plus::internal_new(&mut rng)
    }

    #[inline]
    pub fn new_from(seed: i64) -> Self {
        let mut rng = SplitMix64::new_from(seed);
        XorShift128Plus::internal_new(&mut rng)
    }

    #[inline]
    fn internal_new(rng: &mut SplitMix64) -> Self {
        let mut instance = XorShift128Plus {
            x0: rng.next_long(),
            x1: rng.next_long(),
        };
        instance.escape();
        instance
    }

    #[inline]
    fn escape(&mut self) {
        let mut l: i64 = 0i64;
        for _ in 0..20 {
            l = self.next_long();
        }
        if l == 0i64 {
            black_hole(l as u8);
        }
    }
}