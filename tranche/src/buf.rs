// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use core::mem;

use crate::core::{BasedBufTranche, BufTranche, UnexpectedEndError};

macro_rules! call_for_each_taker {
    ($mac:ident) => {
        call_for_each_taker! {
            $mac

            u16 take_u16_ne take_u16_le take_u16_be
            i16 take_i16_ne take_i16_le take_i16_be

            u32 take_u32_ne take_u32_le take_u32_be
            i32 take_i32_ne take_i32_le take_i32_be

            u64 take_u64_ne take_u64_le take_u64_be
            i64 take_i64_ne take_i64_le take_i64_be

            u128 take_u128_ne take_u128_le take_u128_be
            i128 take_i128_ne take_i128_le take_i128_be

            usize take_usize_ne take_usize_le take_usize_be
            isize take_isize_ne take_isize_le take_isize_be
        }
    };
    ($mac:ident $($ty:ident $ne:ident $le:ident $be:ident)+) => {
        $(
            $mac!($ty "native" $ne from_ne_bytes);
            $mac!($ty "little" $le from_le_bytes);
            $mac!($ty "big" $be from_be_bytes);
        )+
    };
}

macro_rules! taker_with_computed_doc {
    ($(#[doc = $doc:expr])+ #[inline] $($tt:tt)+) => {
        $(#[doc = $doc])+ #[inline] $($tt)+
    };
}

macro_rules! tranche_taker {
    ($ty:ident $endian:tt $take:ident $from:ident) => {
        taker_with_computed_doc! {
            /// Returns a
            #[doc = concat!("`", stringify!($ty), "`")]
            /// by taking the first
            #[doc = concat!("`mem::size_of::<", stringify!($ty), ">()`")]
            /// bytes out of the tranche in
            #[doc = $endian]
            /// endian order.
            ///
            /// Returns `Err(_)` if `self` is not long enough.
            #[inline]
            pub fn $take(&mut self) -> Result<$ty, UnexpectedEndError> {
                const SIZE: usize = mem::size_of::<$ty>();
                let ptr: *const u8 = self.take_front(SIZE)?.as_ptr();
                Ok($ty::$from(unsafe { *(ptr as *const [u8; SIZE]) }))
            }
        }
    };
}

impl BufTranche<'_> {
    /// Takes the first `u8` out of the tranche.
    ///
    /// Returns `Err(_)` if `self` is not long enough.
    pub fn take_u8(&mut self) -> Result<u8, UnexpectedEndError> {
        Ok(*self.take_first()?)
    }

    /// Takes the first `i8` out of the tranche.
    ///
    /// Returns `Err(_)` if `self` is not long enough.
    pub fn take_i8(&mut self) -> Result<i8, UnexpectedEndError> {
        Ok(self.take_u8()? as i8)
    }

    call_for_each_taker!(tranche_taker);
}

macro_rules! based_tranche_taker {
    ($ty:ident $endian:tt $take:ident $from:ident) => {
        taker_with_computed_doc! {
            /// Returns a
            #[doc = concat!("`", stringify!($ty), "`")]
            /// by taking the first
            #[doc = concat!("`mem::size_of::<", stringify!($ty), ">()`")]
            /// bytes out of the tranche in
            #[doc = $endian]
            /// endian order.
            ///
            /// The internal offset is incremented accordingly.
            ///
            /// Returns `Err(_)` if `self` is not long enough.
            #[inline]
            pub fn $take(&mut self) -> Result<$ty, UnexpectedEndError> {
                self.inner.$take()
            }
        }
    };
}

impl BasedBufTranche<'_> {
    /// Takes the first `u8` out of the tranche.
    ///
    /// The internal offset is incremented accordingly.
    ///
    /// Returns `Err(_)` if `self` is not long enough.
    pub fn take_u8(&mut self) -> Result<u8, UnexpectedEndError> {
        self.inner.take_u8()
    }

    /// Takes the first `i8` out of the tranche.
    ///
    /// The internal offset is incremented accordingly.
    ///
    /// Returns `Err(_)` if `self` is not long enough.
    pub fn take_i8(&mut self) -> Result<i8, UnexpectedEndError> {
        self.inner.take_i8()
    }

    call_for_each_taker!(based_tranche_taker);
}
