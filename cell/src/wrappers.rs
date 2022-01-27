use crate::errors::RwCellError;
use crate::refs::{CellRef, CellRefMut, EitherRef, EitherRefMut};
use crate::rw_cell::RwCell;
use std::cell::RefCell;

pub trait ReferenceCell<T> {
    fn try_borrow(&self) -> Option<CellRef<'_, T>>;

    fn try_borrow_mut(&self) -> Option<CellRefMut<'_, T>>;

    fn borrow(&self) -> Result<CellRef<'_, T>, RwCellError>;

    fn borrow_mut(&self) -> Result<CellRefMut<'_, T>, RwCellError>;

    fn borrow_panic(&self) -> CellRef<'_, T>;

    fn borrow_mut_panic(&self) -> CellRefMut<'_, T>;
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
    fn borrow(&self) -> Result<CellRef<'_, T>, RwCellError> {
        match self.try_borrow() {
            Ok(r) => Ok(CellRef {
                ref_: EitherRef::Std(r),
            }),
            Err(_) => Err(RwCellError {}),
        }
    }

    #[inline]
    fn borrow_mut(&self) -> Result<CellRefMut<'_, T>, RwCellError> {
        match self.try_borrow_mut() {
            Ok(r) => Ok(CellRefMut {
                ref_: EitherRefMut::Std(r),
            }),
            Err(_) => Err(RwCellError {}),
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
    fn borrow(&self) -> Result<CellRef<'_, T>, RwCellError> {
        match self.borrow() {
            Ok(r) => Ok(CellRef {
                ref_: EitherRef::Locked(r),
            }),
            Err(e) => Err(e),
        }
    }

    #[inline]
    fn borrow_mut(&self) -> Result<CellRefMut<'_, T>, RwCellError> {
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
