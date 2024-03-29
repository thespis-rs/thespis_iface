# thespis - CHANGELOG


## [Unreleased]

[Unreleased]: https://github.com/najamelan/async_executors/compare/release...dev


## [0.2.0] - 2022-05-30

[0.2.0]: https://github.com/najamelan/async_executors/compare/0.1.1...0.2.0

### Changed
  - __BREAKING__: Actor name is no longer optional. This simplifies the api, and you can always pass an empty string.


## [0.1.1] - 2022-05-22

[0.1.1]: https://github.com/najamelan/async_executors/compare/0.1.0...0.1.1

### Added

  - allow `Indentity` for `?Sized` types.
  - an auto impl for `Address` on `&mut T`.
  - a funding badge.

### Updated
  - switch to rust edition 2021.
  - rustdoc configuration.


## [0.1.0] - 2021-06-20

[0.1.0]: https://github.com/najamelan/async_executors/compare/0.1.0-alpha.3...0.1.0

### Removed

  - the `AsAddress` trait has been incorporated in `Address`.

## [0.1.0-alpha.3] - 2020-02-18

[0.1.0-alpha.3]: https://github.com/najamelan/async_executors/compare/0.1.0-alpha.2...0.1.0-alpha.3

  - Revert UnwindSafe requirements on Address and Message because in practice it is unworkable.

## [0.1.0-alpha.2] - 2020-11-17

[0.1.0-alpha.2]: https://github.com/najamelan/async_executors/compare/0.1.0-alpha.1...0.1.0-alpha.2

  - Require UnwindSafe on Address trait.

## 0.1.0-alpha.1 - 2020-09-10

  - initial release, not for production.




