#![allow(incomplete_features)]
#![feature(adt_const_params, gca_const_items, generic_const_items, gca_min_const_items)]

use diman_unit_system::unit_system_internal;

unit_system_internal!(
    quantity_type Quantity;
    dimension_type Dimension;
    dimension Length;
    #[base(Length)]
    #[symbol(m)]
    unit meters;
    unit kilometers = 1000.0 * meters;
    unit millimeters = 0.001 * meters;
    unit undefined = undefined;
);

fn main() {
    use crate::units::meters;
    let _ = 1.0 * meters;
}
