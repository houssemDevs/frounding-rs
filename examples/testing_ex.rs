extern crate frounding;

use frounding::RoundingState;

// I used this example to make sure that the changes occur at the registers, using the gdb.

fn main() {

    let mut rs = RoundingState::new();

    rs.upward();

    rs.downward();

    rs.to_nearest();

    rs.to_zero();

}
