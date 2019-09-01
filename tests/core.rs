use core::fmt;
use static_assertions::{assert_eq_size, assert_eq_type, assert_impl_all, assert_not_impl_any};
use tranche::{BasedBufTranche, BasedTranche, BufTranche, Tranche, UnexpectedEndError};

#[test]
fn it_compiled() {}

fn _sizes<'a, T>()
where
    T: 'a,
{
    assert_eq_size!(Tranche<'a, T>, &'a [T]);
    assert_eq_size!(Option<Tranche<'a, T>>, Tranche<'a, T>);

    assert_eq_size!(Option<BasedTranche<'a, T>>, BasedTranche<'a, T>);
}

fn _buf_tranche<'a>() {
    assert_eq_type!(BufTranche<'a>, Tranche<'a, u8>);
    assert_eq_type!(BasedBufTranche<'a>, BasedTranche<'a, u8>);
}

fn _tranche_sync<T: Sync>() {
    assert_impl_all!(Tranche<T>, Send, Sync);
    assert_impl_all!(BasedTranche<T>, Send, Sync);
}

fn _tranche_clone<T>() {
    assert_impl_all!(Tranche<T>, Clone);
    assert_not_impl_any!(Tranche<T>, Copy);

    assert_impl_all!(BasedTranche<T>, Clone);
    assert_not_impl_any!(BasedTranche<T>, Copy);
}

fn _tranche_default<T>() {
    assert_impl_all!(Tranche<T>, Default);
    assert_impl_all!(BasedTranche<T>, Default);
}

fn _tranche_debug<T: fmt::Debug>() {
    assert_impl_all!(Tranche<T>, fmt::Debug);
    assert_impl_all!(BasedTranche<T>, fmt::Debug);
}

fn _tranche_iter<T>() {
    assert_impl_all!(Tranche<T>, Iterator);
    assert_impl_all!(BasedTranche<T>, Iterator);
}

fn _error() {
    assert_impl_all!(
        UnexpectedEndError,
        Clone,
        fmt::Debug,
        fmt::Display,
        Send,
        Sync,
    );
}
