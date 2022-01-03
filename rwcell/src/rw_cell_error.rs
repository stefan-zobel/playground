use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result};

pub struct RwCellError {}

impl Error for RwCellError {}

impl Debug for RwCellError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Debug::fmt("RwCellError", f)
    }
}

impl Display for RwCellError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt("already mutably borrowed by current thread", f)
    }
}
