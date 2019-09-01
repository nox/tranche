use static_assertions::assert_impl_all;
use std::error::Error;
use std::io::{BufRead, Read};
use tranche::{BasedBufTranche, BufTranche, UnexpectedEndError};

#[test]
fn it_compiled() {}

fn _read<'a>() {
    assert_impl_all!(BufTranche<'a>, BufRead, Read);
    assert_impl_all!(BasedBufTranche<'a>, BufRead, Read);
}

fn _error() {
    assert_impl_all!(UnexpectedEndError, Error);
}
