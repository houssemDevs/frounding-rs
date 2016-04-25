
extern crate gcc;

fn main() {
	gcc::compile_library("librounding.a", &["csrc/rounding_sse.c","csrc/rounding_fpu.c"]);
}