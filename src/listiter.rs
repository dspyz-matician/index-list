/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! The defintions of the ListIter type
use std::iter::{DoubleEndedIterator, FusedIterator};

use crate::{listindex::ListIndex, IndexList};

/// A double-ended iterator over all the elements in the list. It is fused and
/// can be reversed.
pub struct ListIterG<L> {
    pub(crate) list: L,
    pub(crate) start: ListIndex,
    pub(crate) end: ListIndex,
    pub(crate) len: usize,
}

pub type ListIter<'a, T> = ListIterG<&'a IndexList<T>>;
pub type ListIterMut<'a, T> = ListIterG<&'a mut IndexList<T>>;

impl<L> ListIterG<L> {
    #[inline]
    fn set_empty(&mut self) {
        self.start = ListIndex::new();
        self.end = ListIndex::new();
        self.len = 0;
    }
}

impl<L: GetByIndex> Iterator for ListIterG<L> {
    type Item = L::ItemRef;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        // We're relying on the fact that the list is acyclic for safety.
        // As long as the list is acyclic, it's not possible for the iterator
        // to encounter the same element twice.
        let item = unsafe { self.list.get(self.start) }?;
        if self.start == self.end {
            self.set_empty()
        } else {
            self.start = self.list.next_index(self.start);
            self.len -= 1;
        }
        Some(item)
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }
}
impl<L: GetByIndex> FusedIterator for ListIterG<L> {}
impl<L: GetByIndex> ExactSizeIterator for ListIterG<L> {}

impl<L: GetByIndex> DoubleEndedIterator for ListIterG<L> {
    fn next_back(&mut self) -> Option<Self::Item> {
        // We're relying on the fact that the list is acyclic for safety.
        // As long as the list is acyclic, it's not possible for the iterator
        // to encounter the same element twice.
        let item = unsafe { self.list.get(self.end) }?;
        if self.start == self.end {
            self.set_empty()
        } else {
            self.end = self.list.prev_index(self.end);
            self.len -= 1;
        }
        Some(item)
    }
}

pub trait GetByIndex {
    type ItemRef;

    /// # Safety
    /// For the &'a mut IndexList<T> implementation, a careless caller could
    /// potentially obtain multiple mutable references to the same element by
    /// calling this method multiple times with the same index.
    unsafe fn get(&mut self, index: ListIndex) -> Option<Self::ItemRef>;

    fn next_index(&self, index: ListIndex) -> ListIndex;
    fn prev_index(&self, index: ListIndex) -> ListIndex;
}

impl<'a, T> GetByIndex for &'a IndexList<T> {
    type ItemRef = &'a T;

    unsafe fn get(&mut self, index: ListIndex) -> Option<Self::ItemRef> {
        (**self).get(index)
    }

    fn next_index(&self, index: ListIndex) -> ListIndex {
        (**self).next_index(index)
    }

    fn prev_index(&self, index: ListIndex) -> ListIndex {
        (**self).prev_index(index)
    }
}

impl<'a, T> GetByIndex for &'a mut IndexList<T> {
    type ItemRef = &'a mut T;

    unsafe fn get(&mut self, index: ListIndex) -> Option<Self::ItemRef> {
        let ptr = (**self).get_mut(index)? as *mut T;
        Some(&mut *ptr)
    }

    fn next_index(&self, index: ListIndex) -> ListIndex {
        (**self).next_index(index)
    }

    fn prev_index(&self, index: ListIndex) -> ListIndex {
        (**self).prev_index(index)
    }
}
