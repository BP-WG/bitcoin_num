// Bitcoin Hashes Library
// Written in 2018 by
//   Dr. Maxim Orlovsky <orlovsky@pandoracore.com>
//
// To the extent possible under law, the author(s) have dedicated all
// copyright and related and neighboring rights to this software to
// the public domain worldwide. This software is distributed without
// any warranty.
//
// You should have received a copy of the CC0 Public Domain Dedication
// along with this software.
// If not, see <http://creativecommons.org/publicdomain/zero/1.0/>.
//

//! # Rust Bibtcoin Numeric Library
//!
//! This is a simple, no-dependency library which implements utility functions
//! for working with big integers, hex encodings etc needed by Bitcoin. It
//! differs from other existing cragtes in the field in it's focus on
//! supporting the same versions of Rust compilers as `bitcoin` and
//! `bitcoin_hashes` crates and not using any external dependencies in order
//! to reduce potential attack surface.
//!

// Coding conventions
#![deny(non_upper_case_globals)]
#![deny(non_camel_case_types)]
#![deny(non_snake_case)]
#![deny(unused_mut)]
#![deny(missing_docs)]

// In general, rust is absolutely horrid at supporting users doing things like,
// for example, compiling Rust code for real environments. Disable useless lints
// that don't do anything but annoy us and cant actually ever be resolved.
#![allow(bare_trait_objects)]
#![allow(ellipsis_inclusive_range_patterns)]

#![cfg_attr(all(not(test), not(feature = "std")), no_std)]
#![cfg_attr(all(test, feature = "unstable"), feature(test))]
#[cfg(all(test, feature = "unstable"))] extern crate test;

#[cfg(any(test, feature="std"))] pub extern crate core;

//#[cfg(any(test, feature = "std"))] mod std_impls;
