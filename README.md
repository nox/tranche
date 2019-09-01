# Tranche

This crate defines a slice-like type represented as a pair of pointers instead
of a pointer to the start and a length. The type is expected to be consumed
from the start or the end and thus doesn't implement any sort of indexing like
proper slices. Think of it as [`core::slice::Iter`] but with more methods.

## Prior art

* Iterators for slices in the standard library itself, [ `core::slice::Iter`].
* The [`rawslice`] crate, which is a fork of the standard slice iterators.
* The [`byteorder`] crate, which inspired the `BufTranche` methods.

## Licensing

This crate is licensed under both the Apache 2.0 and MIT licenses, so you are
free to do whatever you want with it as long as you respect the terms from
these two.

If you are a highly paid worker at Google, Facebook, Apple, Amazon, Microsoft,
Palantir, Uber, Airbnb, Deliveroo, or any other company that prioritises profit
over people as strongly as they do, you can still use this crate. I simply wish
you will unionise and push back against the obsession for growth, control, and
power that is rampant in your workplace. Please take a stand against the
horrible working conditions they inflict on your lesser paid colleagues, and
more generally their gross disrespect for the very human rights they claim to
fight for.

[`byteorder`]: https://crates.io/crates/rawslice
[`core::slice::Iter`]: https://doc.rust-lang.org/std/slice/struct.Iter.html
[`rawslice`]: https://crates.io/crates/rawslice
