
#![warn(missing_docs)]

//! frounding is obivously a rust crate. The purpose of this crate is to change the rounding
//! mode of the SSE and FPU unit when doing numeric operation. for some fields like interval arithmetic
//! the rounding mode (aka: rounding direction) is very important.



mod rounding_mode;
pub use rounding_mode::*;

extern "C" {
    fn sse_cw() -> u32;
    fn sse_set_cw(cw: u32);
    fn sse_round_upward();
    fn sse_round_downward();
    fn sse_round_tonearest();
    fn sse_round_truncate();
    fn fpu_cw() -> u32;
    fn fpu_set_cw(cw: u32);
    fn fpu_round_upward();
    fn fpu_round_downward();
    fn fpu_round_tonearest();
    fn fpu_round_truncate();
}

#[derive(Debug,Clone)]
/// RoundingState Keep track of the current rounding mode of both the SSE and the x87 FPU units.
pub struct RoundingState {
    sse_cw: u32,
    fpu_cw: u32,
    sse_rounding: RoundingMode,
    fpu_rounding: RoundingMode,
}


impl RoundingState {
    /// Create new RoundingState struct and fill it with the current status of control regiters.
    /// When going out of scope the initial rounding mode is set.
    pub fn new() -> Self {

        RoundingState {
            sse_cw: unsafe { sse_cw() },
            fpu_cw: unsafe { fpu_cw() },
            sse_rounding: RoundingMode::Default,
            fpu_rounding: RoundingMode::Default,
        }

    }

    /// Return the current rounding mode of both SSE and FPU.
    pub fn current_rounding(&self) -> (RoundingMode, RoundingMode) {
        (self.sse_rounding, self.fpu_rounding)
    }

    /// Return the current rounding mode of the SSE unit.
    pub fn sse_rounding(&self) -> RoundingMode {
        self.sse_rounding
    }

    /// Return the current rounding mode of x87 FPU unit.
    pub fn fpu_rounding(&self) -> RoundingMode {
        self.fpu_rounding
    }

    /// Set the rounding mode toward +infinity for both SSE and x87 FPU.
    pub fn upward(&mut self) {
        self.sse_rounding = RoundingMode::Upward;
        self.fpu_rounding = RoundingMode::Upward;
        unsafe {
            sse_round_upward();
            fpu_round_upward();
        }
    }

    /// Set the rounding mode of the SSE toward +infinity.
    pub fn sse_upward(&mut self) {
        self.sse_rounding = RoundingMode::Upward;
        unsafe {
            sse_round_upward();
        }
    }
    /// Set the rounding mode of the FPU toward +infinity.
    pub fn fpu_upward(&mut self) {
        self.fpu_rounding = RoundingMode::Upward;
        unsafe {
            fpu_round_upward();
        }
    }

    /// Set the rounding mode toward -infinity for both SSE and x87 FPU.
    pub fn downward(&mut self) {
        self.sse_rounding = RoundingMode::Downward;
        self.fpu_rounding = RoundingMode::Downward;
        unsafe {
            sse_round_downward();
            fpu_round_downward();
        }
    }
    /// Set the rounding mode of the SSE toward -infinity.
    pub fn sse_downward(&mut self) {
        self.fpu_rounding = RoundingMode::Downward;
        unsafe {
            fpu_round_downward();
        }
    }
    /// Set the rounding mode of the FPU toward -infinity.
    pub fn fpu_downward(&mut self) {
        self.fpu_rounding = RoundingMode::Downward;
        unsafe {
            fpu_round_downward();
        }
    }

    /// Set the rounding mode to nearest for both SSE and x87 FPU.
    pub fn to_nearest(&mut self) {
        self.sse_rounding = RoundingMode::Nearest;
        self.fpu_rounding = RoundingMode::Nearest;
        unsafe {
            sse_round_tonearest();
            fpu_round_tonearest();
        }
    }
    /// Set the rounding mode of the SSE to nearest
    pub fn sse_to_nearest(&mut self) {
        self.sse_rounding = RoundingMode::Nearest;
        unsafe {
            sse_round_tonearest();
        }
    }
    /// Set the rounding mode of FPU to nearest
    pub fn fpu_to_nearest(&mut self) {
        self.fpu_rounding = RoundingMode::Nearest;
        unsafe {
            fpu_round_tonearest();
        }
    }

    /// Set the rounding mode toward zero (aka truncate) for both SSE and x87 FPU.
    pub fn to_zero(&mut self) {
        self.sse_rounding = RoundingMode::ToZero;
        self.fpu_rounding = RoundingMode::ToZero;
        unsafe {
            sse_round_truncate();
            fpu_round_truncate();
        }
    }
    /// Set the rounding mode of SSE toward zero (aka truncate)
    pub fn sse_to_zero(&mut self) {
        self.sse_rounding = RoundingMode::ToZero;
        unsafe {
            sse_round_truncate();
        }
    }
    /// Set the rounding mode of FPU toward zero (aka truncate)
    pub fn fpu_to_zero(&mut self) {
        self.fpu_rounding = RoundingMode::ToZero;
        unsafe {
            fpu_round_truncate();
        }
    }
    /// Reset the rounding mode to the default one (the one found when calling the new function) for both FPU and SEE.
    pub fn reset(&mut self) {
        unsafe {
            sse_set_cw(self.sse_cw);
            fpu_set_cw(self.fpu_cw);
        }
    }
    /// Reset the rounding mode to the default one (the one found when calling the new function) for SEE.
    pub fn sse_reset(&mut self) {
        unsafe {
            sse_set_cw(self.sse_cw);
        }
    }
    /// Reset the rounding mode to the default one (the one found when calling the new function) for FPU.
    pub fn fpu_reset(&mut self) {
        unsafe {
            fpu_set_cw(self.fpu_cw);
        }
    }
}

/// Drop trait is implimented as when going out of scope the default status of the control registers.
/// is retrived.
impl Drop for RoundingState {
    fn drop(&mut self) {
        unsafe {
            sse_set_cw(self.sse_cw);
            fpu_set_cw(self.fpu_cw);
        }
    }
}
