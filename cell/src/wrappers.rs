use crate::errors::CellError;
use crate::refs::{CellRef, CellRefMut, EitherRef, EitherRefMut};
use crate::rw_cell::RwCell;
use std::cell::RefCell;

pub trait ReferenceCell<T> {
    fn try_borrow(&self) -> Option<CellRef<'_, T>>;

    fn try_borrow_mut(&self) -> Option<CellRefMut<'_, T>>;

    fn borrow(&self) -> Result<CellRef<'_, T>, CellError>;

    fn borrow_mut(&self) -> Result<CellRefMut<'_, T>, CellError>;

    fn borrow_panic(&self) -> CellRef<'_, T>;

    fn borrow_mut_panic(&self) -> CellRefMut<'_, T>;
}

impl<'a, T, R: ReferenceCell<T>> ReferenceCell<T> for &'a R {
    fn try_borrow(&self) -> Option<CellRef<'_, T>> {
        (**self).try_borrow()
    }

    fn try_borrow_mut(&self) -> Option<CellRefMut<'_, T>> {
        (**self).try_borrow_mut()
    }

    fn borrow(&self) -> Result<CellRef<'_, T>, CellError> {
        (**self).borrow()
    }

    fn borrow_mut(&self) -> Result<CellRefMut<'_, T>, CellError> {
        (**self).borrow_mut()
    }

    fn borrow_panic(&self) -> CellRef<'_, T> {
        (**self).borrow_panic()
    }

    fn borrow_mut_panic(&self) -> CellRefMut<'_, T> {
        (**self).borrow_mut_panic()
    }
}

impl<'a, T, R: ReferenceCell<T>> ReferenceCell<T> for &'a mut R {
    fn try_borrow(&self) -> Option<CellRef<'_, T>> {
        (**self).try_borrow()
    }

    fn try_borrow_mut(&self) -> Option<CellRefMut<'_, T>> {
        (**self).try_borrow_mut()
    }

    fn borrow(&self) -> Result<CellRef<'_, T>, CellError> {
        (**self).borrow()
    }

    fn borrow_mut(&self) -> Result<CellRefMut<'_, T>, CellError> {
        (**self).borrow_mut()
    }

    fn borrow_panic(&self) -> CellRef<'_, T> {
        (**self).borrow_panic()
    }

    fn borrow_mut_panic(&self) -> CellRefMut<'_, T> {
        (**self).borrow_mut_panic()
    }
}

impl<T> ReferenceCell<T> for RefCell<T> {
    #[inline]
    fn try_borrow(&self) -> Option<CellRef<'_, T>> {
        match self.try_borrow() {
            Ok(r) => Some(CellRef {
                ref_: EitherRef::Std(r),
            }),
            Err(_) => None,
        }
    }

    #[inline]
    fn try_borrow_mut(&self) -> Option<CellRefMut<'_, T>> {
        match self.try_borrow_mut() {
            Ok(r) => Some(CellRefMut {
                ref_: EitherRefMut::Std(r),
            }),
            Err(_) => None,
        }
    }

    #[inline]
    fn borrow(&self) -> Result<CellRef<'_, T>, CellError> {
        match self.try_borrow() {
            Ok(r) => Ok(CellRef {
                ref_: EitherRef::Std(r),
            }),
            Err(_) => Err(CellError {}),
        }
    }

    #[inline]
    fn borrow_mut(&self) -> Result<CellRefMut<'_, T>, CellError> {
        match self.try_borrow_mut() {
            Ok(r) => Ok(CellRefMut {
                ref_: EitherRefMut::Std(r),
            }),
            Err(_) => Err(CellError {}),
        }
    }

    #[inline]
    fn borrow_panic(&self) -> CellRef<'_, T> {
        CellRef {
            ref_: EitherRef::Std(self.borrow()),
        }
    }

    #[inline]
    fn borrow_mut_panic(&self) -> CellRefMut<'_, T> {
        CellRefMut {
            ref_: EitherRefMut::Std(self.borrow_mut()),
        }
    }
}

impl<T> ReferenceCell<T> for RwCell<T> {
    #[inline]
    fn try_borrow(&self) -> Option<CellRef<'_, T>> {
        self.try_borrow().map(|r| CellRef {
            ref_: EitherRef::Locked(r),
        })
    }

    #[inline]
    fn try_borrow_mut(&self) -> Option<CellRefMut<'_, T>> {
        self.try_borrow_mut().map(|r| CellRefMut {
            ref_: EitherRefMut::Locked(r),
        })
    }

    #[inline]
    fn borrow(&self) -> Result<CellRef<'_, T>, CellError> {
        match self.borrow() {
            Ok(r) => Ok(CellRef {
                ref_: EitherRef::Locked(r),
            }),
            Err(e) => Err(e),
        }
    }

    #[inline]
    fn borrow_mut(&self) -> Result<CellRefMut<'_, T>, CellError> {
        match self.borrow_mut() {
            Ok(r) => Ok(CellRefMut {
                ref_: EitherRefMut::Locked(r),
            }),
            Err(e) => Err(e),
        }
    }

    #[inline]
    fn borrow_panic(&self) -> CellRef<'_, T> {
        CellRef {
            ref_: EitherRef::Locked(self.borrow_panic()),
        }
    }

    #[inline]
    fn borrow_mut_panic(&self) -> CellRefMut<'_, T> {
        CellRefMut {
            ref_: EitherRefMut::Locked(self.borrow_mut_panic()),
        }
    }
}

#[cfg(test)]
fn test_refcell<T: std::fmt::Debug, U: ReferenceCell<T>>(refcell: U) {
    let cell_ref = refcell.try_borrow().unwrap();
    let val = &*cell_ref;
    println!("RefCell value : {:?}", val);
}

#[cfg(test)]
fn test_rw_cell<T: std::fmt::Debug, U: ReferenceCell<T>>(refcell: U) {
    let cell_ref = refcell.try_borrow().unwrap();
    let val = &*cell_ref;
    println!("RwCell value : {:?}", val);
}

#[cfg(test)]
fn test_refcell_by_ref<T: std::fmt::Debug, U: ReferenceCell<T>>(refcell: &U) {
    let cell_ref = refcell.try_borrow().unwrap();
    let val = &*cell_ref;
    println!("RefCell value : {:?}", val);
}

#[cfg(test)]
fn test_rw_cell_by_ref<T: std::fmt::Debug, U: ReferenceCell<T>>(refcell: &U) {
    let cell_ref = refcell.try_borrow().unwrap();
    let val = &*cell_ref;
    println!("RwCell value : {:?}", val);
}

#[cfg(test)]
fn test_refcell_mut<U: ReferenceCell<i64>>(refcell: &mut U) {
    let mut cell_ref = refcell.try_borrow_mut().unwrap();
    let val = &mut *cell_ref;
    println!("RefCell value : {}", *val);
    *val = 49;
}

#[cfg(test)]
fn test_rw_cell_mut<U: ReferenceCell<i64>>(refcell: &mut U) {
    let mut cell_ref = refcell.try_borrow_mut().unwrap();
    let val = &mut *cell_ref;
    println!("RwCell value : {:?}", *val);
    *val = 49;
}

#[cfg(test)]
mod wrapper_tests {
    use super::*;

    #[test]
    fn test_pass_ref_cell() {
        let cell = RefCell::new(42);
        test_refcell(&cell);
    }

    #[test]
    fn test_pass_ref_cell2() {
        let mut cell = RefCell::new(42);
        test_refcell_mut(&mut cell);
        let val = cell.try_borrow().unwrap();
        println!("RefCell new value : {}", *val);
    }

    #[test]
    fn test_pass_ref_cell3() {
        let cell = RefCell::new(42);
        test_refcell(cell);
    }

    #[test]
    fn test_pass_ref_cell4() {
        let cell = RefCell::new(42);
        test_refcell_by_ref(&cell);
    }

    #[test]
    fn test_pass_rw_cell() {
        let cell = RwCell::new(42);
        test_rw_cell(&cell);
    }

    #[test]
    fn test_pass_rw_cell2() {
        let mut cell = RwCell::new(42);
        test_rw_cell_mut(&mut cell);
        let val = cell.try_borrow().unwrap();
        println!("RwCell new value : {}", *val);
    }

    #[test]
    fn test_pass_rw_cell3() {
        let cell = RwCell::new(42);
        test_rw_cell(cell);
    }

    #[test]
    fn test_pass_rw_cell4() {
        let cell = RwCell::new(42);
        test_rw_cell_by_ref(&cell);
    }
}
