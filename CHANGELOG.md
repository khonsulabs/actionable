# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

### Breaking Changes

- This crate now defaults to non-async code. `Dispatcher` has been renamed to
  `AsyncDispatcher`, and a new trait named `Dispatcher` has been added that is
  not async. The `Actionable` derive macro has a new parameter, `async`, which
  generates handler traits that contain async function definitions.

  Updating existing code should be straightforward: add `async` to the
  `Actionable` derive:

  ```rust
  #[derive(Actionable, Debug)]
  #[actionable(async)]
  enum MyRequest {}
  ```

  And switch from deriving `Dispatcher` to deriving `AsyncDispatcher`.

## 0.2.0

### Added

- `Identifier::Bytes` is a new way to represent identifiers that can't be
  represented as a string or a u64. When `Display`ed, the value is shown as
  hexadecimal with a preceding '$'.

### Changed

- `Identifier` now implements `Hash` and `Eq` in a way that all variants can
  consistently be tested for equality.

  - `Identifier::Any` only equals `Identifier::Any`
  - `Identifier::Integer` is compared against all other variants' byte
    representations using `u64::to_be_bytes()`
  - `Identifier::String` is compared against all other variants' byte
    representations using `str::as_bytes()`
  - `Identifier::Bytes` is compared against all other variants' byte
    representations.

## 0.1.1

### Changed

- `Statement` and `ActionNameList` now implement `Clone`.

## 0.1.0

### Breaking Changes

- `Statement::allow_all()` has been renamed to `Statement::allow_all_for_any_resource()`.
- `Statement::actions` has been wrapped in an `Option`.
- `Statement::configuration` is a new field that can contain configuration
  values. This can be used to grant different configuration values to different
  roles. For example, an API request rate limit could be granted on a per-user
  basis using this feature.

### Added

- `Statement` now includes a builder-style interface to make constructing statements more fluid.
- `Permissions::get()` is a new method that can be used to look up configuration
  values for a resource name.

## 0.1.0-rc.3

### Added

- Added new `check()` function that automatically returns a `PermissionDenied`
  error based on the parameters provided.
