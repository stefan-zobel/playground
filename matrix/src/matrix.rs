use std::ops::{Index, IndexMut, Mul, MulAssign};

struct MatrixD<const ROWS: usize, const COLS: usize> {
    rows: usize,
    cols: usize,
    a: Box<[[f64; COLS]; ROWS]>,
}

// Implement `Index` for shared references to a `MatrixD`.
impl<'a, const ROWS: usize, const COLS: usize> Index<usize> for &'a MatrixD<ROWS, COLS> {
    type Output = [f64; COLS];
    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.a[index]
    }
}

// Implement `Index` for exclusive references to a `MatrixD`.
impl<'a, const ROWS: usize, const COLS: usize> Index<usize> for &'a mut MatrixD<ROWS, COLS> {
    type Output = [f64; COLS];
    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.a[index]
    }
}

// Implement `IndexMut` for references to a `MatrixD`.
impl<'a, const ROWS: usize, const COLS: usize> IndexMut<usize> for &'a mut MatrixD<ROWS, COLS> {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.a[index]
    }
}

impl<const ROWS_LEFT: usize, const COLS_LEFT: usize, const COLS_RIGHT: usize>
    Mul<MatrixD<COLS_LEFT, COLS_RIGHT>> for MatrixD<ROWS_LEFT, COLS_LEFT>
{
    type Output = MatrixD<ROWS_LEFT, COLS_RIGHT>;

    fn mul(self, rhs: MatrixD<COLS_LEFT, COLS_RIGHT>) -> Self::Output {
        // to do ...
        MatrixD::<ROWS_LEFT, COLS_RIGHT>::new()
    }
}

impl<const ROWS: usize> MulAssign<Self> for MatrixD<ROWS, ROWS> {
    fn mul_assign(&mut self, rhs: Self) {
        // to do ...
    }
}

impl<const ROWS: usize, const COLS: usize> Index<usize> for MatrixD<ROWS, COLS> {
    type Output = [f64; COLS];

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.a[index]
    }
}

impl<const ROWS: usize, const COLS: usize> IndexMut<usize> for MatrixD<ROWS, COLS> {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.a[index]
    }
}

impl<const ROWS: usize, const COLS: usize> MatrixD<ROWS, COLS> {
    #[inline]
    pub fn new() -> Self {
        MatrixD::new_on_heap()
    }

    #[inline]
    fn new_on_heap() -> Self {
        let slice = vec![[0.0f64; COLS]; ROWS].into_boxed_slice();
        let ptr = Box::into_raw(slice) as *mut [[f64; COLS]; ROWS];
        let box_ = unsafe { Box::from_raw(ptr) };
        MatrixD {
            rows: ROWS,
            cols: COLS,
            a: box_,
        }
    }
}

#[cfg(test)]
mod matrix_tests {
    use super::*;

    #[test]
    fn test_create() {
        let _m = MatrixD::<1500, 1500>::new();
        println!("rows: {} ", _m.rows);
        println!("cols: {} ", _m.cols);
    }

    #[test]
    fn test_mul() {
        let a = MatrixD::<1500, 1500>::new();
        let b = MatrixD::<1500, 9>::new();
        let c = a * b;
        println!("c rows: {} ", c.rows);
        println!("c cols: {} ", c.cols);
    }

    #[test]
    fn test_mul_assign() {
        let mut a = MatrixD::<1500, 1500>::new();
        let b = MatrixD::<1500, 1500>::new();
        a *= b;
        println!("a rows: {} ", a.rows);
        println!("a cols: {} ", a.cols);
    }

    #[test]
    fn test_one_index() {
        let mut a = MatrixD::<10, 10>::new();
        let row = a[0];
        println!("row = {:?}", row);
    }

    #[test]
    fn test_two_indices() {
        let mut a = MatrixD::<10, 10>::new();
        let elem = a[0][0];
        println!("a[0][0] = {}", elem);
    }

    #[test]
    fn test_one_index_assign() {
        let row = [9.5f64; 10];
        let mut a = MatrixD::<10, 10>::new();
        a[0] = row;
        let row2 = a[0];
        println!("row after = {:?}", row2);
    }

    #[test]
    fn test_two_indices_assign() {
        let mut a = MatrixD::<10, 10>::new();
        a[0][0] = 199.99;
        let elem = a[0][0];
        println!("a[0][0] = {}", elem);
    }
}
