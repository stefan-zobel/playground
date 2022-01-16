mod java_glue;
mod jni_c_header;
pub use crate::java_glue::*;
pub use log::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
