Unreleased
----------
- Adjusted `Id::new` to panic when a `usize` counter overflow is
  detected


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
