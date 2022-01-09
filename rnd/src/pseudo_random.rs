use crate::seed::black_hole;
use crate::xor_shift_128plus::XorShift128Plus;

const DOUBLE_NORM: f64 = 1.0f64 / (1i64 << 53) as f64;
const FLOAT_NORM: f32 = 1.0f32 / (1i32 << 24) as f32;

pub trait PseudoRandom {
    fn next_long(&mut self) -> i64;

    #[inline]
    fn next_int(&mut self) -> i32 {
        ((self.next_long() as u64 >> 32) as i64) as i32
    }

    #[inline]
    fn next_double(&mut self) -> f64 {
        ((self.next_long() as u64 >> 11) as i64) as f64 * DOUBLE_NORM
    }

    #[inline]
    fn next_float(&mut self) -> f32 {
        ((self.next_long() as u64 >> 40) as i64) as f32 * FLOAT_NORM
    }

    #[inline]
    fn next_bool(&mut self) -> bool {
        self.next_long() < 0i64
    }
}

pub struct Stc64 {
    s0: i64,
    s1: i64,
    s2: i64,
    s3: i64,
    seq: i64,
}

impl PseudoRandom for Stc64 {
    #[inline]
    fn next_long(&mut self) -> i64 {
        let xb = self.s1;
        let xc = self.s2;

        self.s3 = self.s3.wrapping_add(self.seq);
        let rnd = (self.s0 ^ self.s3).wrapping_add(xb);

        self.s0 = xb ^ (xb as u64 >> 11) as i64;
        self.s1 = xc.wrapping_add(xc << 3);
        self.s2 = ((xc << 24) | (xc as u64 >> 40) as i64).wrapping_add(rnd);

        rnd
    }
}

impl Stc64 {
    #[inline]
    pub fn new() -> Self {
        Stc64::internal_new(XorShift128Plus::new().next_long())
    }

    #[inline]
    pub fn new_from(seed: i64) -> Self {
        Stc64::internal_new(XorShift128Plus::new_from(seed).next_long())
    }

    #[inline]
    fn internal_new(seed: i64) -> Self {
        let mut instance = Stc64 {
            s0: seed,
            s1: seed.wrapping_add(0x26aa069ea2fb1a4di64),
            s2: seed.wrapping_add(0x70c72c95cd592d04i64),
            s3: seed.wrapping_add(0x504f333d3aa0b359i64),
            // seq must be odd
            seq: (((seed.wrapping_add(0x3504f333d3aa0b37i64)) << 1) | 1i64),
        };
        instance.escape();
        instance
    }

    #[inline]
    fn escape(&mut self) {
        let mut l: i64 = 0i64;
        for _ in 0..12 {
            l = self.next_long();
        }
        if l == 0i64 {
            black_hole(l as u8);
        }
    }
}

impl Default for Stc64 {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}
