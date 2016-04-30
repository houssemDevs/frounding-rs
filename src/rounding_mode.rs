
//! This module define just an enum to represent diffrent possible rounding directions.

use std::fmt;
use std::fmt::Display;

#[derive(Debug,Clone,Copy)]
/// Rounding direction
pub enum RoundingMode {
    /// The default rounding when starting the program.
    Default,
    /// Round upward ( toward positive infinity +inf )
    Upward,
    /// Round downward ( toward negative infinity -inf )
    Downward,
    /// Round to nearest
    Nearest,
    /// Round tward zero ( truncate )
    ToZero,
}

impl Display for RoundingMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RoundingMode::Default => write!(f, "Default Rounding"),
            RoundingMode::Upward => write!(f, "Upward Rounding"),
            RoundingMode::Downward => write!(f, "Downward Rounding"),
            RoundingMode::Nearest => write!(f, "To nearest Rounding"),
            RoundingMode::ToZero => write!(f, "Toward Zero Rounding"),
        }
    }
}
