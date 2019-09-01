// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use core::iter::FusedIterator;

use crate::core::{BasedTranche, Tranche};

impl<'a, T> Iterator for Tranche<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.take_first().ok()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }

    fn count(self) -> usize {
        self.len()
    }
}

impl<'a, T> Iterator for BasedTranche<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }

    fn count(self) -> usize {
        self.inner.count()
    }
}

impl<T> ExactSizeIterator for Tranche<'_, T> {
    fn len(&self) -> usize {
        self.len()
    }
}

impl<T> ExactSizeIterator for BasedTranche<'_, T> {
    fn len(&self) -> usize {
        self.inner.len()
    }
}

impl<T> FusedIterator for Tranche<'_, T> {}

impl<T> FusedIterator for BasedTranche<'_, T> {}
