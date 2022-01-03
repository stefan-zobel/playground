#![feature(thread_id_value)]

pub mod lock;
pub mod rw_cell;
pub mod rw_cell_error;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
