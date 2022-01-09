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

pub struct XoShiRo256StarStar {
    x0: i64,
    x1: i64,
    x2: i64,
    x3: i64,
}

impl PseudoRandom for XoShiRo256StarStar {
    #[inline]
    fn next_long(&mut self) -> i64 {
        let s1 = self.x1;
        let t = s1 << 17;
        let x = s1.wrapping_add(s1 << 2);
        let mut rnd = (x << 7) | (x as u64 >> 57) as i64;
        rnd = rnd.wrapping_add(rnd << 3);

        self.x2 ^= self.x0;
        self.x3 ^= s1;
        self.x1 ^= self.x2;
        let s3 = self.x3;
        self.x0 ^= s3;

        self.x2 ^= t;
        self.x3 = (s3 << 45) | (s3 as u64 >> 19) as i64;

        rnd
    }
}

impl XoShiRo256StarStar {
    #[inline]
    pub fn new() -> Self {
        XoShiRo256StarStar::internal_new(&mut XorShift128Plus::new())
    }

    #[inline]
    pub fn new_from(seed: i64) -> Self {
        XoShiRo256StarStar::internal_new(&mut XorShift128Plus::new_from(seed))
    }

    #[inline]
    fn internal_new(seeder: &mut XorShift128Plus) -> Self {
        let mut instance = XoShiRo256StarStar {
            x0: seeder.next_long(),
            x1: seeder.next_long(),
            x2: seeder.next_long(),
            x3: seeder.next_long(),
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

impl Default for XoShiRo256StarStar {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

/*
 * Multiplier used in the LCG portion of the algorithm. Chosen based on
 * research by Sebastiano Vigna and Guy Steele (2019). The spectral scores
 * for dimensions 2 through 8 for the multiplier 0xd1342543de82ef95 are
 * [0.958602, 0.937479, 0.870757, 0.822326, 0.820405, 0.813065, 0.760215].
 */
const M: i64 = 0xd1342543de82ef95u64 as i64;

pub struct Lcg64Xor1024Mix {
    /*
     * The parameter that is used as an additive constant for the LCG.
     * Must be odd.
     */
    a: i64,
    /*
     * The per-instance state: s for the LCG; the array seed for the XBG;
     * pos is the rotating pointer into the array seed. At least one of
     * the 16 elements of the array seed must be nonzero.
     */
    s: i64,
    pos: usize,
    seed: [i64; 16],
}

impl PseudoRandom for Lcg64Xor1024Mix {
    #[inline]
    fn next_long(&mut self) -> i64 {
        todo!()
    }
}

impl Lcg64Xor1024Mix {
    pub fn new() -> Self {
        todo!()
    }
}

impl Default for Lcg64Xor1024Mix {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}
