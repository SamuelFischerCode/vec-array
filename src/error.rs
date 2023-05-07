use std::fmt;
use thiserror::Error;

/// Just an error
#[derive(Error, Debug, Copy, Clone, Default, PartialEq)]
pub struct ArrTooSmall;

impl fmt::Display for ArrTooSmall {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}
