pub mod bit_mix;
pub mod seed;

#[cfg(test)]
mod lib_tests {
    use crate::seed::raw_seed;

    #[test]
    fn test_raw_seed() {
        let raw_seed = raw_seed();
        println!("raw seed: {}", raw_seed);
        assert_ne!(raw_seed, 0);
    }
}
