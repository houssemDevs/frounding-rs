
#![feature(asm)]


mod rounding_mode;
pub use rounding_mode::*;

extern {
	fn sse_cw() -> u32;
	fn round_upward();
	fn round_downward();
	fn round_tonearest();
	fn round_truncate();
	fn set_sse_cw(cw: u32);
}

#[derive(Debug,Clone)]
pub struct RoundingState {
    sse_cw: u32,
    current_round: RoundingMode,
}


impl RoundingState {
    pub fn new() -> Self {
        let sse_dflt_cw: u32 = unsafe {
           sse_cw()
        };

        RoundingState {
            sse_cw: sse_dflt_cw,
            current_round: RoundingMode::Default,
        }

    }
    
    pub fn current_rounding(&self) -> RoundingMode {
    	self.current_round
    }
    
    pub fn upward(&mut self) {
    	self.current_round = RoundingMode::Upward;
    	unsafe {
    		round_upward();
    	}
    }
    
    pub fn downward(&mut self) {
    	self.current_round = RoundingMode::Downward;
    	unsafe {
    		round_downward();
    	}
    }
    
    pub fn to_nearest(&mut self) {
    	self.current_round = RoundingMode::Nearest;
    	unsafe {
    		round_tonearest();
    	}
    }
    
    pub fn truncate(&mut self) {
    	self.current_round = RoundingMode::Truncate;
    	unsafe {
    		round_truncate();
    	}
    }
    
}


impl Drop for RoundingState {
	fn drop(&mut self) {
		unsafe {
			set_sse_cw(self.sse_cw);
		}
	}
}

