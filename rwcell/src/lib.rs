#![feature(thread_id_value)]
#![warn(rust_2018_idioms)]

pub(crate) mod lock;
pub mod rw_cell;
pub mod rc_graph;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
