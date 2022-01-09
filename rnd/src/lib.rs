pub mod bit_mix;
pub mod pseudo_random;
mod seed;
mod split_mix64;
mod split_mix64_seed;
mod xor_shift_128plus;

#[cfg(test)]
mod lib_tests {
    use crate::pseudo_random::Stc64;
    use crate::seed::raw_seed;
    use crate::split_mix64::SplitMix64;
    use crate::xor_shift_128plus::XorShift128Plus;

    #[test]
    fn test_stc64() {
        let mut prng = Stc64::new();
        for _ in 0..6 {
            println!("{}", prng.next_long());
        }
        assert_ne!(prng.next_long(), prng.next_long());
    }

    #[test]
    fn test_raw_seed() {
        let raw_seed = raw_seed();
        println!("raw seed: {}", raw_seed);
        assert_ne!(raw_seed, 0);
    }

    #[test]
    fn test_split_mix64() {
        let mut rng = SplitMix64::new();
        let l = rng.next_long();
        let i = rng.next_int();
        println!("SplitMix64 64 bit signed: {}", l);
        println!("SplitMix64 32 bit signed: {}", i);
        assert_ne!(l, i as i64);
    }

    #[test]
    fn test_xor_shift_128plus() {
        let mut rng1 = XorShift128Plus::new_from(123i64);
        let mut rng2 = XorShift128Plus::new();
        for i in 1..4 {
            let rand = rng1.next_long();
            println!("rng1: i = {} : {}", i, rand);
            println!("rng2: i = {} : {}", i, rng2.next_long());
            if i == 3 {
                assert_eq!(rand, -4974179587566018337i64);
            }
        }
        assert_ne!(rng1.next_long(), rng2.next_long());
    }
}
