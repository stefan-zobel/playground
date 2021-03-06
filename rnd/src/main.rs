// Copyright 2022 Stefan Zobel
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use rnd::pseudo_random::{
    Lcg64Xor1024Mix, PseudoRandom, Stc64, ThreadLocalPrng, XoShiRo256StarStar,
};
use std::time::Instant;

fn main() {
    let mut rand = 0.0f64;
    let seed = 0xcb24d0a5c88c35b3u64 as i64;
    let mut prng0 = ThreadLocalPrng::get();
    let mut bytes: [u8; 9] = [0; 9];
    prng0.next_bytes(&mut bytes);
    println!("random bytes: {:?}", bytes);
    let mut normal: [f64; 2] = [-99.99; 2];
    prng0.next_gaussians(&mut normal);
    println!("standard normal: {:?}", normal);

    let mut prng1 = Lcg64Xor1024Mix::new_from(seed);
    let start = Instant::now();
    for _ in 0..100_000_000 {
        rand = prng1.next_double();
    }
    let millis_elapsed = start.elapsed().as_millis();
    println!("Lcg64Xor1024Mix took {} ms", millis_elapsed);
    println!("last rand: {}", rand);
    let mut prng2 = XoShiRo256StarStar::new_from(seed);
    let start = Instant::now();
    for _ in 0..100_000_000 {
        rand = prng2.next_double();
    }
    let millis_elapsed = start.elapsed().as_millis();
    println!("XoShiRo256StarStar took {} ms", millis_elapsed);
    println!("last rand: {}", rand);
    let mut prng3 = Stc64::new_from(seed);
    let start = Instant::now();
    for _ in 0..100_000_000 {
        rand = prng3.next_double();
    }
    let millis_elapsed = start.elapsed().as_millis();
    println!("Stc64 took {} ms", millis_elapsed);
    println!("last rand: {}", rand);
    let mut prng4 = ThreadLocalPrng::get();
    let start = Instant::now();
    for _ in 0..100_000_000 {
        rand = prng4.next_double();
    }
    let millis_elapsed = start.elapsed().as_millis();
    println!("ThreadLocalPrng took {} ms", millis_elapsed);
    println!("last rand: {}", rand);
}
