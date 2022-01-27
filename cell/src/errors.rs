use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

pub(crate) const ERR_MSG: &str = "already mutably borrowed by current thread";

pub struct CellError {}

impl Error for CellError {}

impl Debug for CellError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt("RwCellError", f)
    }
}

impl Display for CellError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(ERR_MSG, f)
    }
}
