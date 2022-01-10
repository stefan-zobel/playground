use rnd::pseudo_random::{Lcg64Xor1024Mix, PseudoRandom};
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
    println!("took {} ms", duration.as_millis());
    println!("last rand: {}", rand);
}
