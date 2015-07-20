// Copyright © 2015 Michael Howell <michael@notriddle.com>
// This work is free. It comes without any warranty, to the extent permitted
// by applicable law. You can redistribute it and/or modify it under the terms
// of the Do What The Fuck You Want To Public License, Version 2, as published
// by Sam Hocevar. See the COPYING file, or <http://www.wtfpl.net/> for
// more details.

//! Empty collection and iterator.

extern crate void;
use void::Void;
use std::borrow::{Borrow, BorrowMut};
use std::hash::{Hash, Hasher};
use std::iter::FromIterator;
use std::ops::{Deref, DerefMut, Index, IndexMut};
use std::slice;

/// A collection that is empty at creation and cannot be modified.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct List;

/// An iterator that never yields anything.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Iter;

impl<T> Borrow<[T]> for List {
    #[inline]
    fn borrow(&self) -> &[T] {
        unsafe { slice::from_raw_parts(0x1 as *const T, 0) }
    }
}

impl<T> BorrowMut<[T]> for List {
    #[inline]
    fn borrow_mut(&mut self) -> &mut [T] {
        unsafe { slice::from_raw_parts_mut(0x1 as *mut T, 0) }
    }
}

impl<'a> From<List> for &'a [Void] {
    #[inline]
    fn from(_: List) -> &'a [Void] {
        unsafe { slice::from_raw_parts(0x1 as *const Void, 0) }
    }
}

impl<'a, 'b> From<&'a List> for &'b [Void] {
    #[inline]
    fn from(_: &'a List) -> &'b [Void] {
        unsafe { slice::from_raw_parts(0x1 as *const Void, 0) }
    }
}

impl<'a> From<&'a [Void]> for List {
    #[inline]
    fn from(_: &'a [Void]) -> List {
        List
    }
}

impl FromIterator<Void> for List {
    #[inline]
    fn from_iter<I: IntoIterator<Item=Void>>(_: I) -> List {
        List
    }
}

impl<'a> FromIterator<&'a Void> for List {
    #[inline]
    fn from_iter<I: IntoIterator<Item=&'a Void>>(_: I) -> List {
        List
    }
}

impl Hash for List {
    #[inline]
    fn hash<H: Hasher>(&self, _: &mut H) {
        // This body intentionally left blank
    }
}

impl<U> Index<U> for List {
    type Output = Void;
    #[inline]
    fn index(&self, _: U) -> &Void {
        panic!("index into empty::List")
    }
}

impl<U> IndexMut<U> for List {
    #[inline]
    fn index_mut(&mut self, _: U) -> &mut Void {
        panic!("index_mut into empty::List")
    }
}

impl Deref for List {
    type Target = [Void];
    #[inline]
    fn deref(&self) -> &[Void] {
        self.borrow()
    }
}

impl DerefMut for List {
    #[inline]
    fn deref_mut(&mut self) -> &mut [Void] {
        self.borrow_mut()
    }
}

impl AsRef<List> for List {
    fn as_ref(&self) -> &List {
        self
    }
}

impl<T> AsRef<[T]> for List {
    fn as_ref(&self) -> &[T] {
        self.borrow()
    }
}

impl<'a> IntoIterator for &'a List {
    type Item = Void;
    type IntoIter = Iter;
    #[inline]
    fn into_iter(self) -> Iter {
        Iter
    }
}

impl IntoIterator for List {
    type Item = Void;
    type IntoIter = Iter;
    #[inline]
    fn into_iter(self) -> Iter {
        Iter
    }
}

impl Iterator for Iter {
    type Item = Void;
    #[inline]
    fn next(&mut self) -> Option<Void> {
        None
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(0))
    }
    #[inline]
    fn count(self) -> usize {
        0
    }
    #[inline]
    fn last(self) -> Option<Void> {
        None
    }
    #[inline]
    fn nth(&mut self, _: usize) -> Option<Void> {
        None
    }
}

impl DoubleEndedIterator for Iter {
    #[inline]
    fn next_back(&mut self) -> Option<Void> {
        None
    }
}

impl ExactSizeIterator for Iter {
    #[inline]
    fn len(&self) -> usize {
        0
    }
}

#[cfg(test)]
mod test {
    use void;
    #[test]
    fn it_works() {
        let l = super::List;
        for i in &*l {
            void::unreachable(*i);
        }
    }
    #[test]
    fn it_works_safe() {
        let l = super::List;
        for _ in &*l {
            panic!();
        }
    }
    #[test]
    fn len() {
        let l = super::List;
        assert_eq!(l.len(), 0);
    }
}
