#![allow(incomplete_features)]
#![feature(adt_const_params, gca_const_items, generic_const_items, gca_min_const_items)]

use diman_unit_system::unit_system_internal;

unit_system_internal!(
    quantity_type Quantity;
    dimension_type Dimension;
    dimension Length;
    unit meters = 0.001 * kilometers;
    unit kilometers = 1000.0 * meters;
);

fn main() {}
