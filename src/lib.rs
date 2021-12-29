// lib.rs

// *************************************************************************
// * Copyright (C) 2018-2021 Daniel Mueller (deso@posteo.net)              *
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
#![warn(
  future_incompatible,
  missing_copy_implementations,
  missing_debug_implementations,
  missing_docs,
  rust_2018_compatibility,
  rust_2018_idioms,
  trivial_casts,
  trivial_numeric_casts,
  unsafe_op_in_unsafe_fn,
  unstable_features,
  unused_import_braces,
  unused_qualifications,
  unused_results
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
use core::num::NonZeroU16;
use core::num::NonZeroU32;
use core::num::NonZeroU64;
use core::num::NonZeroU8;
use core::num::NonZeroUsize;
use core::sync::atomic::AtomicU16;
use core::sync::atomic::AtomicU32;
use core::sync::atomic::AtomicU64;
use core::sync::atomic::AtomicU8;
use core::sync::atomic::AtomicUsize;
use core::sync::atomic::Ordering;


macro_rules! IdImpl {
  ( $(#[$docs:meta])* struct $name: ident, $int_type:ty, $non_zero_type:ty, $atomic_type: ty ) => {
    $(#[$docs])*
    #[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[repr(transparent)]
    pub struct $name<T>
    where
      T: Copy,
    {
      id: $non_zero_type,
      phantom: PhantomData<T>,
    }

    impl<T> $name<T>
    where
      T: Copy,
    {
      /// Create a new ID using the given value.
      ///
      /// # Safety
      /// - `id` must not be zero
      /// - `id` should be unique with respect to other IDs created for this
      ///   `T` to preserve the invariant that IDs are unique
      #[inline]
      unsafe fn new_unchecked(id: $int_type) -> Self {
        Self {
          id: unsafe { <$non_zero_type>::new_unchecked(id) },
          phantom: PhantomData,
        }
      }

      /// Create a new unique ID.
      #[inline]
      pub fn new() -> Self {
        static NEXT_ID: $atomic_type = <$atomic_type>::new(1);

        let id = NEXT_ID.fetch_add(1, Ordering::Relaxed);
        assert_ne!(
          id, 0,
          "overflow detected; please use a larger integer to or reconsider your use case"
        );

        // SAFETY: The provided ID cannot be 0 (unless we overflow, in which
        //         case we have other problems). We ensure uniqueness
        //         because we increment IDs and this is the only constructor
        //         for ID objects.
        unsafe { Self::new_unchecked(id) }
      }

      /// Retrieve the underlying integer value.
      #[inline]
      pub fn get(self) -> $int_type {
        self.id.get()
      }
    }

    impl<T> Default for $name<T>
    where
      T: Copy,
    {
      /// Create a new unique ID.
      #[inline]
      fn default() -> Self {
        Self::new()
      }
    }

    impl<T> Debug for $name<T>
    where
      T: Copy,
    {
      #[inline]
      fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct(stringify!($name))
          .field("id", &self.id)
          .finish()
      }
    }

    impl<T> Display for $name<T>
    where
      T: Copy,
    {
      /// Format the ID with the given formatter.
      #[inline]
      fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.id)
      }
    }
  }
}


IdImpl! {
  /// A struct representing IDs usable for various purposes.
  ///
  /// Except for [`Clone`], [`Copy`], [`Debug`], and [`Display`] which are
  /// implemented unconditionally, the type will only implement [`Eq`],
  /// [`Ord`], [`PartialEq`], [`PartialOrd`], and [`Hash`] if the provided
  /// `T` implements them.
  /// Note furthermore that we want all ID objects to be lightweight and,
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
  struct Id, usize, NonZeroUsize, AtomicUsize
}
IdImpl! {
  /// A struct representing IDs usable for various purposes using an eight
  /// bit wide unsigned integer.
  ///
  /// Please see the [`Id`] type for more general information and usage
  /// examples.
  struct IdU8, u8, NonZeroU8, AtomicU8
}
IdImpl! {
  /// A struct representing IDs usable for various purposes using an 16
  /// bit wide unsigned integer.
  ///
  /// Please see the [`Id`] type for more general information and usage
  /// examples.
  struct IdU16, u16, NonZeroU16, AtomicU16
}
IdImpl! {
  /// A struct representing IDs usable for various purposes using an 32
  /// bit wide unsigned integer.
  ///
  /// Please see the [`Id`] type for more general information and usage
  /// examples.
  struct IdU32, u32, NonZeroU32, AtomicU32
}
IdImpl! {
  /// A struct representing IDs usable for various purposes using an 64
  /// bit wide unsigned integer.
  ///
  /// Please see the [`Id`] type for more general information and usage
  /// examples.
  struct IdU64, u64, NonZeroU64, AtomicU64
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


  /// Make sure that [`Id`] values are increasing.
  #[test]
  fn unique_id_increases() {
    let id1 = TestId::new();
    let id2 = TestId::new();

    assert!(id2 > id1);
    assert!(id2.get() > id1.get());
  }

  /// Test that [`Id`] objects created on different threads preserve
  /// uniqueness invariant.
  #[test]
  fn thread_safety() {
    #[allow(clippy::needless_collect)]
    fn test<T>()
    where
      T: FromIterator<TestId> + IntoIterator,
    {
      let handles = (0..100).map(|_| spawn(TestId::new)).collect::<Vec<_>>();

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

  /// Check that the [`Debug`] implementation of [`Id`] works as
  /// expected.
  #[test]
  fn debug() {
    let id = unsafe { TestId::new_unchecked(42) };
    assert_eq!(format!("{:?}", id), "Id { id: 42 }");

    type TestId2 = IdU16<()>;

    let id = unsafe { TestId2::new_unchecked(1337) };
    assert_eq!(format!("{:?}", id), "IdU16 { id: 1337 }");
  }

  /// Check that the [`Display`] implementation of [`Id`] works as
  /// expected.
  #[test]
  fn display() {
    let id = unsafe { TestId::new_unchecked(43) };
    assert_eq!(format!("{}", id), "43");
  }

  /// Make sure that our [`Id`] type has expected memory layout and
  /// size.
  #[test]
  fn size() {
    let id = Some(TestId::new());
    assert_eq!(size_of_val(&id), size_of::<TestId>());
    assert_eq!(size_of::<TestId>(), size_of::<usize>());

    assert_eq!(size_of::<IdU8<()>>(), size_of::<u8>());
    assert_eq!(size_of::<IdU16<()>>(), size_of::<u16>());
    assert_eq!(size_of::<IdU32<()>>(), size_of::<u32>());
    assert_eq!(size_of::<IdU64<()>>(), size_of::<u64>());
  }
}
