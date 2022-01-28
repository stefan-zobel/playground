pub mod lock;
mod owner;

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
