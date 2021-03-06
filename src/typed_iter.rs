use crate::prelude::*;
///
#[derive(Clone, Debug)]
pub struct TypedIter<'a, T: 'a> {
    pub(crate) inner: Iter<'a, T>,
}

impl<'a, T> Iterator for TypedIter<'a, T> {
    type Item = (TypedIndex<T>, &'a T);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        match self.inner.next() {
            Some((idx, el)) => Some((idx.into(), el)),
            None => None,
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl<'a, T> DoubleEndedIterator for TypedIter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        match self.inner.next_back() {
            Some((idx, el)) => Some((idx.into(), el)),
            None => None,
        }
    }
}