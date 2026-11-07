#![allow(incomplete_features)]
#![feature(
    adt_const_params,
    gca_const_items,
    generic_const_items,
    gca_min_const_items
)]

pub mod example_system;
pub mod utils;

mod float;

mod type_aliases;

pub mod unit_aliases;

#[cfg(feature = "f64")]
mod debug;

#[cfg(feature = "f64")]
mod dimension_defs;

#[cfg(feature = "si")]
#[cfg(feature = "f64")]
mod gas;

#[cfg(feature = "glam")]
mod glam;

#[cfg(feature = "mpi")]
mod mpi;

#[cfg(feature = "serde")]
mod serde;

#[cfg(feature = "rand")]
mod rand;

#[cfg(feature = "rational-dimensions")]
pub mod rational_dimensions;
