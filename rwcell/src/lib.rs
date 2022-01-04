#![feature(thread_id_value)]

pub(crate) mod lock;
pub mod rw_cell;
pub mod rw_cell_error;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
