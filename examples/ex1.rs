#![feature(asm)]

extern crate frounding;

use frounding::*;

fn main() {
	let (a,mut b): (f64,f64) = (1f64,3f64);
	let mut c: f64 = a/b;
	println!("{:.100}",c);
    {
    	let mut cr = RoundingState::new();
    	cr.upward();
    	println!("{:.100}",b);
    	b = 3f64;
    	cr.downward();
		println!("{:.100}",b);
    }
    c = a / b;
    println!("{:.100}",c);
}
