[![Build Status](https://travis-ci.org/jedisct1/rust-coarsetime.svg?branch=master)](https://travis-ci.org/jedisct1/rust-coarsetime?branch=master)
[![Windows build status](https://ci.appveyor.com/api/projects/status/xlbhk9850dvl5ylh?svg=true)](https://ci.appveyor.com/project/jedisct1/rust-coarsetime)
# coarsetime

A Rust crate to make time measurements that focuses on speed.

This crate is a partial replacement for the `Time` and `Duration` structures
from the standard library, with the following differences:

* Speed is privileged over accuracy. In particular, `CLOCK_MONOTONIC_COARSE` is
used to retrieve the clock value on Linux systems, and transformations avoid
operations that can be slow on non-Intel systems.
* The number of system calls can be kept to a minimum. The "most recent
timestamp" is always kept in memory. It can be read at zero cost, and can be
updated only as frequently as necessary.

# Installation

`coarsetime` is available on [crates.io](https://crates.io/crates/coarsetime)
and works on Rust stable, beta, and nightly.

Windows and Unix-like systems are supported.

Available features:

* `nightly`: optimizes for rust-nightly
