#![allow(incomplete_features)]
#![feature(adt_const_params, gca_const_items, generic_const_items, gca_min_const_items)]
pub mod example_system;

fn main() {
    use example_system::dimensions::Length;
    Length::unknown_method(49.0);
}
