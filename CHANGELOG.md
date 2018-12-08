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
