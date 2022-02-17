# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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
