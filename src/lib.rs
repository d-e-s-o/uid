// lib.rs

// *************************************************************************
// * Copyright (C) 2018 Daniel Mueller (deso@posteo.net)                   *
// *                                                                       *
// * This program is free software: you can redistribute it and/or modify  *
// * it under the terms of the GNU General Public License as published by  *
// * the Free Software Foundation, either version 3 of the License, or     *
// * (at your option) any later version.                                   *
// *                                                                       *
// * This program is distributed in the hope that it will be useful,       *
// * but WITHOUT ANY WARRANTY; without even the implied warranty of        *
// * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the         *
// * GNU General Public License for more details.                          *
// *                                                                       *
// * You should have received a copy of the GNU General Public License     *
// * along with this program.  If not, see <http://www.gnu.org/licenses/>. *
// *************************************************************************

#![no_std]
#![deny(
  future_incompatible,
  missing_debug_implementations,
  missing_docs,
  rust_2018_compatibility,
  rust_2018_idioms,
  unstable_features,
  unused_import_braces,
  unused_qualifications,
  unused_results,
  warnings,
)]

//! A crate providing in-memory IDs. Among others, the IDs are
//! guaranteed to be unique, even when created on different threads.

#[cfg(test)]
#[macro_use]
extern crate std;

use core::fmt::Debug;
use core::fmt::Display;
use core::fmt::Formatter;
use core::fmt::Result;
use core::marker::PhantomData;
use core::num::NonZeroUsize;
use core::sync::atomic::AtomicUsize;
use core::sync::atomic::Ordering;


/// A struct representing IDs usable for various purposes.
///
/// Note that Rust only truly provides the implementations of the
/// various traits we derive from when `T` also provides them. Note
/// furthermore that we want all ID objects to be lightweight and,
/// hence, require the implementation of `Copy` for `T` (which we do not
/// for all the other, optional, traits).
///
/// # Examples
///
/// A commonly seen pattern for creating of a type `Id` that is unique
/// may look as follows:
/// ```rust
/// use uid::Id as IdT;
///
/// #[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
/// struct T(());
///
/// type Id = IdT<T>;
///
/// let id1 = Id::new();
/// let id2 = Id::new();
///
/// assert_ne!(id1, id2)
/// ```
///
/// In this example the type `T` is just an arbitrary type, but it
/// allows us to create distinct ID types. For example, when another ID
/// type is required for a different purpose, that can be easily
/// created:
/// ```rust
/// # use uid::Id as IdT;
/// # #[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
/// # struct T(());
/// # type Id = IdT<T>;
/// #[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
/// struct U(());
///
/// type Key = IdT<U>;
///
/// // `Key` and `Id` are fundamentally different types, with no
/// // allowed interaction between each other. That is, Rust's type
/// // system will prevent accidental usage of one in place of the
/// // other. The same can be said about the relationship to built-in
/// // numeric types such as `usize` or `u64`.
/// ```
#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Id<T>
where
  T: Copy,
{
  id: NonZeroUsize,
  phantom: PhantomData<T>,
}

impl<T> Id<T>
where
  T: Copy,
{
  /// Create a new `Id` using the given value.
  fn new_unchecked(id: usize) -> Self {
    Id {
      id: unsafe { NonZeroUsize::new_unchecked(id) },
      phantom: PhantomData,
    }
  }

  /// Create a new unique `Id`.
  pub fn new() -> Self {
    static NEXT_ID: AtomicUsize = AtomicUsize::new(1);

    let id = NEXT_ID.fetch_add(1, Ordering::Relaxed);
    Self::new_unchecked(id)
  }

  /// Retrieve the underlying `usize` value.
  pub fn get(self) -> usize {
    self.id.get()
  }
}

impl<T> Debug for Id<T>
where
  T: Copy,
{
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    f.debug_struct("Id").field("id", &self.id).finish()
  }
}

impl<T> Display for Id<T>
where
  T: Copy,
{
  /// Format the `Id` into the given formatter.
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    write!(f, "{}", self.id)
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  use std::collections::BTreeSet;
  use std::collections::HashSet;
  use std::iter::FromIterator;
  use std::mem::size_of;
  use std::mem::size_of_val;
  use std::thread::spawn;
  use std::vec::Vec;


  type TestId = Id<u32>;


  #[test]
  fn unique_id_increases() {
    let id1 = TestId::new();
    let id2 = TestId::new();

    assert!(id2 > id1);
    assert!(id2.get() > id1.get());
  }

  #[test]
  fn thread_safety() {
    fn test<T>()
    where
      T: FromIterator<TestId> + IntoIterator,
    {
      let handles = (0..100)
        .map(|_| spawn(|| TestId::new()))
        .collect::<Vec<_>>();

      let result = handles
        .into_iter()
        .map(|x| x.join().unwrap())
        .collect::<T>();

      assert_eq!(result.into_iter().count(), 100);
    }

    // Run the test both with a `BTreeSet` and `HashSet` to test the
    // implementations of the traits they require.
    test::<BTreeSet<TestId>>();
    test::<HashSet<TestId>>();
  }

  #[test]
  fn debug() {
    let id = TestId::new_unchecked(42);
    assert_eq!(format!("{:?}", id), "Id { id: 42 }");
  }

  #[test]
  fn display() {
    let id = TestId::new_unchecked(43);
    assert_eq!(format!("{}", id), "43");
  }

  #[test]
  fn size() {
    let id = Some(TestId::new());
    assert_eq!(size_of_val(&id), size_of::<TestId>());
  }
}
