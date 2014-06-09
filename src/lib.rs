#![crate_id = "interstellar"]
//#![deny(missing_doc)]
#![deny(unused_result)]
#![deny(deprecated_owned_vector)]
#![deny(unnecessary_qualification)]
#![feature(globs)]

//! A fast 2d particle physics library for simple and pretty
//! simulations.

pub mod draw;
pub mod field;
pub mod emitter;
pub mod particle;
pub mod vector;
pub mod state;
