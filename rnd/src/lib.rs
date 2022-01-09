pub mod bit_mix;
pub mod seed;
pub mod split_mix64;
pub mod split_mix64_seed;

#[cfg(test)]
mod lib_tests {
    use crate::seed::raw_seed;
    use crate::split_mix64::SplitMix64;

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
}
