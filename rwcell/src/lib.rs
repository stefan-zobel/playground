#![feature(thread_id_value)]
#![warn(rust_2018_idioms)]

pub(crate) mod lock;
pub mod rw_cell;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
