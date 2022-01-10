use rnd::pseudo_random::{Lcg64Xor1024Mix, PseudoRandom, Stc64, XoShiRo256StarStar};
use std::time::Instant;

fn main() {
    let mut rand = 0.0f64;
    let seed = 0xcb24d0a5c88c35b3u64 as i64;
    let mut prng = Lcg64Xor1024Mix::new_from(seed);
    let start = Instant::now();
    for _ in 0..100_000_000 {
        rand = prng.next_double();
    }
    let stop = Instant::now();
    let duration = stop.duration_since(start);
    println!("Lcg64Xor1024Mix took {} ms", duration.as_millis());
    println!("last rand: {}", rand);
    let mut prng2 = XoShiRo256StarStar::new_from(seed);
    let start = Instant::now();
    for _ in 0..100_000_000 {
        rand = prng2.next_double();
    }
    let stop = Instant::now();
    let duration = stop.duration_since(start);
    println!("XoShiRo256StarStar took {} ms", duration.as_millis());
    println!("last rand: {}", rand);
    let mut prng3 = Stc64::new_from(seed);
    let start = Instant::now();
    for _ in 0..100_000_000 {
        rand = prng3.next_double();
    }
    let stop = Instant::now();
    let duration = stop.duration_since(start);
    println!("Stc64 took {} ms", duration.as_millis());
    println!("last rand: {}", rand);
}
