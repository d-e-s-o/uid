Unreleased
----------
- Bumped minimum supported Rust version to `1.58`


0.1.8
-----
- Switched to using GitHub Actions as CI provider


0.1.7
-----
- Made `new_unchecked` constructor public


0.1.6
-----
- Adjusted `Id::new` to panic when a `usize` counter overflow is
  detected
- Removed `T: Copy` requirement for `Id` type
- Annotated `Id` type with `#[repr(transparent)]`
- Adjusted `Debug` representation to use tuple formatting
- Bumped minimum supported Rust version to `1.34`


0.1.5
-----
- Downgraded `deny` crate-level lints to `warn`
- Adjusted pipeline to collect code coverage
  - Added badge indicating showing code coverage percentage
- Excluded unnecessary files from being contained in release bundle


0.1.4
-----
- Enabled CI pipeline comprising building, testing, and linting of the
  project
- Added badges indicating pipeline status, current `crates.io` published
  version of the crate, current `docs.rs` published version of the
  documentation, and minimum version of `rustc` required


0.1.3
-----
- Adjusted crate to use Rust Edition 2018
- Implement `Default` trait for `Id` struct
- Removed `#![deny(warnings)]` attribute and demoted lints prone to
  future changes from `deny` to `warn`


0.1.2
-----
- Made the crate `no_std` compatible
- Added categories to `Cargo.toml`


0.1.1
-----
- Made implementation more friendly to size optimizations (e.g., as
  employed by `Option`) by using `NonZeroUsize` internally


0.1.0
-----
- Initial release
