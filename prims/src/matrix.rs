use std::marker::PhantomData;
use std::ops::{Index, IndexMut};

struct MatrixD<'a, const ROWS: usize, const COLS: usize> {
    a: [[f64; COLS]; ROWS],
    phantom: PhantomData<&'a f64>,
}

struct MatrixDRow<'a, const COLS: usize> {
    row: &'a [f64; COLS],
}

/*
impl<'a, const ROWS: usize, const COLS: usize> Index<usize> for MatrixD<'a, { ROWS }, { COLS } > {
    type Output = MatrixDRow<'a, COLS>;

    fn index(&self, index: usize) -> &'a Self::Output {
        &MatrixDRow{row: &self.a[index]}
    }
}
*/

/*
impl<'a, const ROWS: usize, const COLS: usize> Index<usize> for MatrixD<'a, { ROWS }, { COLS } > {
    type Output = [f64; COLS];

    fn index(&self, index: usize) -> &'a Self::Output {
        &self.a[index]
    }
}
*/

impl<const ROWS: usize, const COLS: usize> Index<usize> for MatrixD<'_, { ROWS }, { COLS }> {
    type Output = [f64; COLS];

    fn index(&self, index: usize) -> &Self::Output {
        &self.a[index]
    }
}

impl<const ROWS: usize, const COLS: usize> IndexMut<usize> for MatrixD<'_, { ROWS }, { COLS }> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.a[index]
    }
}

/*
impl<const COLS: usize> Index<usize> for [f64; COLS] {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
    }
}
*/

impl<const ROWS: usize, const COLS: usize> MatrixD<'_, ROWS, COLS> {
    pub fn new() -> Self {
        MatrixD {
            a: [[0.0f64; COLS]; ROWS],
            phantom: PhantomData,
        }
    }

    pub fn multiply<const COLS_OTHER: usize>(
        &self,
        b: MatrixD<COLS, COLS_OTHER>,
    ) -> MatrixD<ROWS, COLS_OTHER> {
        // to do
        MatrixD::<ROWS, COLS_OTHER>::new()
    }

    pub fn test_set(&mut self, i: usize, j: usize, val: f64) {
        self.a[i][j] = val;
    }
}

#[cfg(test)]
mod matrix_tests {
    use super::*;

    #[test]
    fn test_create() {
        let _m = MatrixD::<2, 3>::new();
    }

    #[test]
    fn test_first_index() {
        let a = MatrixD::<2, 3>::new();
        let row = a[0];
        let _x = row[0];
        let _y = a[0][0];
    }

    #[test]
    fn test_index() {
        let mut i = 1.0f64;
        let mut m = MatrixD::<2, 3>::new();
        for row in 0..2 {
            for col in 0..3 {
                //m.test_set(row, col, i);
                m[row][col] = i;
                i += 1.0f64;
            }
        }
        for i in 0..2 {
            for j in 0..3 {
                let val = m.a[i][j];
                println!("i, j : ({i},{j}) : value: {val}");
                let x = &(m.a[i][j]);
                println!("{:?}", x as *const _);
            }
        }
    }

    #[test]
    fn test_multiply() {
        let a = MatrixD::<2, 3>::new();
        let b = MatrixD::<3, 2>::new();
        let _c = a.multiply(b);
    }

    #[test]
    fn test_memory_layout() {
        let m = MatrixD::<2, 3>::new();
        for i in 0..2 {
            for j in 0..3 {
                let val = m.a[i][j];
                println!("i, j : ({i},{j}) : value: {val}");
                let x = &(m.a[i][j]);
                println!("{:?}", x as *const _);
            }
        }
        //        i, j : (0,0)
        //        i, j : (0,1)
        //        0xb9d7feec0
        //        i, j : (0,2)
        //        0xb9d7feec8
        //        i, j : (1,0)
        //        0xb9d7feed0
        //        i, j : (1,1)
        //        0xb9d7feed8
        //        i, j : (1,2)
        //        0xb9d7feee0
    }

    #[test]
    fn test_multiply2() {
        let _a = MatrixD::<2, 3>::new();
        let _b = MatrixD::<4, 4>::new();
        // this doesn't compile (and it shouldn't)
        //        let c = a.multiply(b);
    }
}
