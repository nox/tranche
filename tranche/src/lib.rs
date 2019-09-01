// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! # Tranche
//!
//! See [`Tranche<'_, T>`](struct.Tranche.html) for what this crate does.
//!
//! This crate is `no_std` by default, the `std` feature provides:
//!
//! * an implementation of `std::error::Error` for
//!   [`UnexpectedEndError`](struct.UnexpectedEndError.html);
//! * an implementation of `std::io::Read` and `std::io::BufRead` for
//!   [`BufTranche<'_>`](type.BufTranche.html) and
//!   [`BasedBufTranche<'_>`](type.BasedBufTranche.html);
//! * an implementation of `From<UnexpectedEndError>` for `std::io::Error`.

#![cfg_attr(not(feature = "std"), no_std)]
#![deny(unsafe_code)]

#[allow(unsafe_code)]
mod core;

#[allow(unsafe_code)]
mod buf;

#[forbid(unsafe_code)]
mod iter;

#[cfg(feature = "std")]
#[forbid(unsafe_code)]
mod std;

pub use self::core::{BasedBufTranche, BasedTranche, BufTranche, Tranche, UnexpectedEndError};
