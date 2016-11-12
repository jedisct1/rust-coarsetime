
#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]
#![cfg_attr(feature="nightly", feature(const_fn))]
#![cfg_attr(feature="nightly", feature(integer_atomics))]

#[macro_use]
extern crate lazy_static;
extern crate libc;

pub use duration::*;
pub use instant::*;
pub use updater::*;

mod duration;
mod helpers;
mod instant;
mod updater;

#[cfg(test)]
mod tests;
