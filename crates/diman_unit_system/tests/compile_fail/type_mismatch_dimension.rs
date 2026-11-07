#![allow(incomplete_features)]
#![feature(adt_const_params, gca_const_items, generic_const_items, gca_min_const_items)]
pub mod example_system;
use example_system::units::meters;
use example_system::dimensions::Time;

fn main() {
    let x: Time<f64> = meters.new(1.0);
}
