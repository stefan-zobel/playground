use crate::seed::black_hole;
use crate::xor_shift_128plus::XorShift128Plus;

pub struct Stc64 {
    s0: i64,
    s1: i64,
    s2: i64,
    s3: i64,
    inc: i64,
    seq: i64,
}

impl Stc64 {
    pub fn next_long(&mut self) -> i64 {
        // todo
        -1
    }

    pub fn new() -> Self {
        Stc64::internal_new(XorShift128Plus::new().next_long())
    }

    pub fn new_from(seed: i64) -> Self {
        Stc64::internal_new(XorShift128Plus::new_from(seed).next_long())
    }

    fn internal_new(seed: i64) -> Self {
        let mut instance = Stc64 {
            s0: seed,
            s1: seed.wrapping_add(0x26aa069ea2fb1a4di64),
            s2: seed.wrapping_add(0x70c72c95cd592d04i64),
            s3: seed.wrapping_add(0x504f333d3aa0b359i64),
            inc: 0i64,
            // seq must be odd
            seq: (((seed.wrapping_add(0x3504f333d3aa0b37i64)) << 1) | 1i64),
        };
        instance.escape();
        instance
    }

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
    fn default() -> Self {
        Self::new()
    }
}
