
const DOUBLE_NORM: f64 = 1.0f64 / (1i64 << 53) as f64;

#[derive(Debug, Clone)]
pub struct Stc64 {
    s0: u64,
    s1: u64,
    s2: u64,
    s3: u64,
    seq: u64,
}

#[derive(Debug, Clone)]
pub struct RomuTrio {
    s0: u64,
    s1: u64,
    s2: u64,
}

impl RomuTrio {
    #[inline]
    pub fn next_long(&mut self) -> i64 {
        let xp = self.s0;
        let yp = self.s1;
        let zp = self.s2;
        self.s0 = 15241094284759029579u64.wrapping_mul(zp);
        self.s1 = (yp.wrapping_add(xp.wrapping_neg())).rotate_left(12);
//        self.s1 = self.s1.rotate_left(12);
        self.s2 = (zp.wrapping_add(yp.wrapping_neg())).rotate_left(44);
//        self.s2 = self.s2.rotate_left(44);
        xp as i64
    }

    /// Returns a uniformly distributed 64-bit floating point value.
    #[inline]
    pub fn next_double(&mut self) -> f64 {
        ((self.next_long() as u64 >> 11) as i64) as f64 * DOUBLE_NORM
    }

    #[inline]
    pub fn new(_seed: i64) -> Self {
        // Test only
        RomuTrio {
            s0: 7156418303300921761u64,
            s1: -6964649053621277904i64 as u64,
            s2: -5128121336112732016i64 as u64,
        }
    }
}

impl Stc64 {
    #[inline]
    pub fn next_long(&mut self) -> i64 {
        let xb = self.s1;
        let xc = self.s2;

        self.s3 = self.s3.wrapping_add(self.seq);
        let rnd = (self.s0 ^ self.s3).wrapping_add(xb);

        self.s0 = xb ^ (xb >> 11);
        self.s1 = xc.wrapping_add(xc << 3);
        self.s2 = xc.rotate_left(24).wrapping_add(rnd);

        rnd as i64
    }

    /// Returns a uniformly distributed 64-bit floating point value.
    #[inline]
    pub fn next_double(&mut self) -> f64 {
        ((self.next_long() as u64 >> 11) as i64) as f64 * DOUBLE_NORM
    }

    #[inline]
    pub fn new(seed: i64) -> Self {
        let seed = seed as u64;
        let mut instance = Stc64 {
            s0: seed,
            s1: seed.wrapping_add(0x26aa069ea2fb1a4du64),
            s2: seed.wrapping_add(0x70c72c95cd592d04u64),
            s3: seed.wrapping_add(0x504f333d3aa0b359u64),
            // seq must be odd
            seq: (((seed.wrapping_add(0x3504f333d3aa0b37u64)) << 1) | 1u64),
        };
        for _ in 0..6 {
            instance.next_long();
        }
        //        instance.escape();
        instance
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn it_works() {
        let n: u64 = 0xa24baed4963ee407u64;
        let x = (n << 24) | (n >> 40);
        let y = n.rotate_left(24);
        assert_eq!(x, y);
        println!("{x}, {y}");
    }

    #[test]
    fn test_time() {
        let mut rand = 0.0f64;
        let seed = 0xcb24d0a5c88c35b3u64 as i64;
        // RomuTrio 1st run
        let mut prng1 = RomuTrio::new(seed);
        let start = Instant::now();
        for _ in 0..2_000_000_000 {
            rand = prng1.next_double();
        }
        let millis_elapsed = start.elapsed().as_millis();
        println!("RomuTrio took {} ms", millis_elapsed);
        println!("last rand: {}", rand);
        println!("next rand: {}", prng1.next_long());
        println!("next rand: {}", prng1.next_long());
        println!("next rand: {}", prng1.next_long());
        // Stc64 1st run
        let mut prng2 = Stc64::new(seed);
        let start = Instant::now();
        for _ in 0..2_000_000_000 {
            rand = prng2.next_double();
        }
        let millis_elapsed = start.elapsed().as_millis();
        println!("Stc64 took {} ms", millis_elapsed);
        println!("last rand: {}", rand);
        println!("next rand: {}", prng2.next_long());
        println!("next rand: {}", prng2.next_long());
        println!("next rand: {}", prng2.next_long());
        let mut _rand = 0.0f64;
        // Stc64 2nd run
        let start = Instant::now();
        for _ in 0..2_000_000_000 {
            _rand = prng2.next_double();
        }
        let millis_elapsed = start.elapsed().as_millis();
        println!("Stc64 took {} ms", millis_elapsed);
        // RomuTrio 2nd run
        let start = Instant::now();
        for _ in 0..2_000_000_000 {
            _rand = prng1.next_double();
        }
        let millis_elapsed = start.elapsed().as_millis();
        println!("RomuTrio took {} ms", millis_elapsed);
        // Stc64 3rd run
        let start = Instant::now();
        for _ in 0..2_000_000_000 {
            _rand = prng2.next_double();
        }
        let millis_elapsed = start.elapsed().as_millis();
        println!("Stc64 took {} ms", millis_elapsed);
        // RomuTrio 3rd run
        let start = Instant::now();
        for _ in 0..2_000_000_000 {
            _rand = prng1.next_double();
        }
        let millis_elapsed = start.elapsed().as_millis();
        println!("RomuTrio took {} ms", millis_elapsed);

        println!("last rand: {}", _rand);
    }
}
