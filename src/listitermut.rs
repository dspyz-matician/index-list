/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! The defintions of the ListIterMut type
use std::iter::{DoubleEndedIterator, FusedIterator};

use crate::{listindex::ListIndex, IndexList};

/// A double-ended iterator over all the elements in the list. It is fused and
/// can be reversed.
pub struct ListIterMut<'a, T> {
    pub(crate) list: &'a mut IndexList<T>,
    pub(crate) start: ListIndex,
    pub(crate) end: ListIndex,
    pub(crate) len: usize,
}

impl<T> ListIterMut<'_, T> {
    #[inline]
    fn set_empty(&mut self) {
        self.start = ListIndex::new();
        self.end = ListIndex::new();
        self.len = 0;
    }
}

impl<'a, T> Iterator for ListIterMut<'a, T> {
    type Item = &'a mut T;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let item = self.list.get_mut(self.start)? as *mut T;
        if self.start == self.end {
            self.set_empty()
        } else {
            self.start = self.list.next_index(self.start);
            self.len -= 1;
        }
        // We're relying on the fact that the list is acyclic for safety.
        // As long as the list is acyclic, it's not possible for the iterator
        // to encounter the same element twice.
        Some(unsafe { &mut *item })
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }
}
impl<T> FusedIterator for ListIterMut<'_, T> {}
impl<T> ExactSizeIterator for ListIterMut<'_, T> {}

impl<T> DoubleEndedIterator for ListIterMut<'_, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let item = self.list.get_mut(self.end)? as *mut T;
        if self.start == self.end {
            self.set_empty()
        } else {
            self.end = self.list.prev_index(self.end);
            self.len -= 1;
        }
        // We're relying on the fact that the list is acyclic for safety.
        // As long as the list is acyclic, it's not possible for the iterator
        // to encounter the same element twice.
        Some(unsafe { &mut *item })
    }
}
