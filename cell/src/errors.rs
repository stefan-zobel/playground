use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

pub(crate) const ERR_MSG: &str = "already mutably borrowed by current thread";

pub struct RwCellError {}

impl Error for RwCellError {}

impl Debug for RwCellError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt("RwCellError", f)
    }
}

impl Display for RwCellError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(ERR_MSG, f)
    }
}
