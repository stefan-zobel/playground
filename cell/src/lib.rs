//#![allow(dead_code)]
#![warn(rust_2018_idioms)]

pub mod arc_sync_cell;
pub mod errors;
pub mod lock;
mod owner;
pub mod refs;
pub mod rw_cell;
pub mod wrappers;

#[cfg(test)]
mod tests {
    use crate::lock::Lock;

    #[test]
    fn test_debug_output() {
        let l = Lock::new(42);
        let _guard = l.write_exclusive();
        println!("Lock: {:?}", l);
    }
}
