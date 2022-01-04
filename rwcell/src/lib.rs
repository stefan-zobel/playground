#![feature(thread_id_value)]

pub(crate) mod lock;
pub mod rw_cell;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
