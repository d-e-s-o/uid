// Copyright (C) 2018-2025 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: GPL-3.0-or-later

extern crate uid;

use uid::Id;

type TestId = Id<u32>;


#[test]
fn distinct_ids() {
  let id1 = TestId::new();
  let id2 = TestId::new();

  assert_ne!(id1, id2);
  assert_ne!(id1.get(), id2.get());
}
