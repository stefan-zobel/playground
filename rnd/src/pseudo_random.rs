use crate::bit_mix::lea_mix64;
use crate::seed::black_hole;
use crate::xor_shift_128plus::XorShift128Plus;

const DOUBLE_NORM: f64 = 1.0f64 / (1i64 << 53) as f64;
const FLOAT_NORM: f32 = 1.0f32 / (1i32 << 24) as f32;

/// A generator of uniform pseudorandom values.
/// <p>
/// Implementors need to supply an implementation of the next_long() method.
/// </p>
/// The default implementations in this trait are efficient for methods that need
/// at least 33 bits of randomness but somehow wasteful for the other methods because
/// it dissipates valuable random bits piled up in the call to next_long() whenever
/// less than 33 random bits are needed for the result type.
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

/// The 256-bit generator `Stc64` is Tyge Løvset's improved variation of
/// `Sfc64`. See
/// <https://github.com/tylov/STC/blob/master/include/stc/crandom.h>.
/// <p>
/// This generator has a guaranteed period of at least 2<sup>64</sup>
/// and an average period of 2<sup>255</sup>.
/// <p>
/// This is the fastest generator supplied in this crate.
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

/// 256-bit `xoshiro256**` pseudo random generator suggested by
/// <a href=https://arxiv.org/pdf/1805.01407.pdf>David Blackman and Sebastiano
/// Vigna (2019)</a>. It is about 40% faster than `XorShift64Star` despite
/// having a 4 times larger state space.
/// <p>
/// This generator has a period of 2<sup>256</sup>&nbsp;&minus;&nbsp;1.
/// <p>
/// This generator is 4-dimensionally equidistributed.
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

/// The `L64X1024MixRandom` algorithm from JDK 17 which uses a linear
/// congruential generator (LCG) as a first subgenerator and a Xor-based
/// generator (xoroshiro1024) as a second subgenerator and then applies
/// a 64-bit mixing function identified by Doug Lea.
/// <p>
/// This generator has a 1088-bit state and a period of
/// 2<sup>64</sup>(2<sup>1024</sup>&minus;1).
/// <p>
/// This generator is 16-dimensionally equidistributed.
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
        // xoroshiro1024: part 1
        let p = self.pos;
        let mut s15 = self.seed[p];
        self.pos = (p + 1) & 15;
        let s0 = self.seed[self.pos];
        // compute result
        let rnd = lea_mix64(self.s.wrapping_add(s0));
        // update LCG sub-generator
        self.s = (M.wrapping_mul(self.s)).wrapping_add(self.a);
        // xoroshiro1024: part 2
        s15 ^= s0;
        self.seed[p] = ((s0 << 25) | (s0 as u64 >> 39) as i64) ^ s15 ^ (s15 << 27);
        self.seed[self.pos] = (s15 << 36) | (s15 as u64 >> 28) as i64;
        rnd
    }
}

impl Lcg64Xor1024Mix {
    #[inline]
    pub fn new() -> Self {
        Lcg64Xor1024Mix::internal_new(&mut XorShift128Plus::new())
    }

    #[inline]
    pub fn new_from(seed: i64) -> Self {
        Lcg64Xor1024Mix::internal_new(&mut XorShift128Plus::new_from(seed))
    }

    #[inline]
    fn internal_new(seeder: &mut XorShift128Plus) -> Self {
        let mut instance = Lcg64Xor1024Mix {
            a: seeder.next_long(),
            s: seeder.next_long(),
            pos: 15usize,
            seed: [0i64; 16],
        };
        for i in 0..16 {
            instance.seed[i] = seeder.next_long();
        }
        instance
    }
}

impl Default for Lcg64Xor1024Mix {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}
