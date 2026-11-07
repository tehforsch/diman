#![allow(incomplete_features)]
#![feature(adt_const_params, gca_const_items, generic_const_items, gca_min_const_items)]
pub mod example_system;
use example_system::units::*;

fn main() {
    let x: () = meters.new(1.0) * meters.new(1.0);
}
