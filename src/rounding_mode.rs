

use std::fmt;
use std::fmt::Display;

#[derive(Debug,Clone,Copy)]
pub enum RoundingMode {
	Default,
	Upward,
	Downward,
	Nearest,
	ToZero
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
