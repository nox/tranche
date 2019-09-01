// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use core::cmp;
use core::str;

use std::error::Error;
use std::io;

use crate::{BasedBufTranche, BufTranche, UnexpectedEndError};

impl io::Read for BufTranche<'_> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let slice = self
            .take_front(cmp::max(self.len(), buf.len()))
            .unwrap()
            .as_slice();
        let len = slice.len();
        if len == 1 {
            buf[0] = slice[0];
        } else {
            buf[..len].copy_from_slice(slice);
        }
        Ok(len)
    }
}

impl io::Read for BasedBufTranche<'_> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.inner.read(buf)
    }
}

impl io::BufRead for BufTranche<'_> {
    fn fill_buf(&mut self) -> io::Result<&[u8]> {
        Ok(self.as_slice())
    }

    fn consume(&mut self, len: usize) {
        if let Err(err) = self.take_front(len) {
            panic!("{}", err);
        }
    }
}

impl io::BufRead for BasedBufTranche<'_> {
    fn fill_buf(&mut self) -> io::Result<&[u8]> {
        self.inner.fill_buf()
    }

    fn consume(&mut self, len: usize) {
        self.inner.consume(len)
    }
}

impl From<UnexpectedEndError> for io::Error {
    fn from(error: UnexpectedEndError) -> Self {
        io::Error::new(io::ErrorKind::UnexpectedEof, error)
    }
}

impl Error for UnexpectedEndError {
    fn description(&self) -> &str {
        "unexpected end"
    }
}
