use crate::{bit_mix::rrxmrrxmsx, bit_mix::xnasam, bit_mix::stafford_mix04, split_mix64_seed::seed, split_mix64_seed::seed_from};

// the golden ratio scaled to 64 bits
const GOLDEN: i64 = 0x9e3779b97f4a7c15u64 as i64;

pub struct SplitMix64 {
    state: i64,
    // Weyl generator step value
    gamma: i64,
}

impl SplitMix64 {
    #[inline]
    pub fn new() -> Self {
        SplitMix64::internal_new(seed())
    }

    #[inline]
    pub fn new_from(seed: i64) -> Self {
        SplitMix64::internal_new(seed_from(seed))
    }

    #[inline]
    pub fn next_long(&mut self) -> i64 {
        self.state = self.state.wrapping_add(self.gamma);
        xnasam(self.state)
    }

    #[inline]
    pub fn next_int(&mut self) -> i32 {
        self.state = self.state.wrapping_add(self.gamma);
        stafford_mix04(self.state)
    }

    #[inline]
    fn internal_new(seed: i64) -> Self {
        SplitMix64 {
            state: seed,
            gamma: mix_gamma(seed.wrapping_add(GOLDEN)),
        }
    }
}

#[inline]
const fn mix_gamma(mut v: i64) -> i64 {
    // force v to be odd
    v = rrxmrrxmsx(v) | 1i64;
    // try to support enough 01 and 10 transitions
    let n = (v ^ (v as u64 >> 1) as i64).count_ones();
    if n < 24 {
        v ^ 0xaaaaaaaaaaaaaaaau64 as i64
    } else {
        v
    }
}
