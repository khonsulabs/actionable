# actionable

[![crate version](https://img.shields.io/crates/v/actionable.svg)](https://crates.io/crates/actionable)
[![Live Build Status](https://img.shields.io/github/workflow/status/khonsulabs/actionable/Tests/main)](https://github.com/khonsulabs/actionable/actions?query=workflow:Tests)
[![HTML Coverage Report for `main` branch](https://khonsulabs.github.io/actionable/coverage/badge.svg)](https://khonsulabs.github.io/actionable/coverage/)
[![Documentation for `main` branch](https://img.shields.io/badge/docs-main-informational)](https://khonsulabs.github.io/actionable/main/actionable/)

Actionable provides the basic functionality needed to build an async-based
API that has a flexible permissions system integrated.

This crate was designed to be used by [`BonsaiDb`](https://bonsaidb.io/)
internally, and as a way for users of `BonsaiDb` to extend their database
servers with their own APIs.

## Permissions

The [`Permissions`](https://khonsulabs.github.io/actionable/main/actionable/struct.Permissions.html) struct is constructed from a list of [`Statement`](https://khonsulabs.github.io/actionable/main/actionable/struct.Statement.html)s. The
`Statement` struct is inspired by [statements in
IAM](https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies_elements_statement.html).
By default, all actions are denied for all resources.

The [`ResourceName`](https://khonsulabs.github.io/actionable/main/actionable/struct.ResourceName.html) struct describes a unique name/id of *anything* in your
application. This is meant to be similar to [ARNs in
IAM](https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_identifiers.html#identifiers-arns),
but instead of being restricted to a format by this library, you are able to
define your own syntax.

The [`Action`](https://khonsulabs.github.io/actionable/main/actionable/trait.Action.html) trait is derive-able, and will convert any enum to something
that can be permitted or denied to any `ResourceName`. This derive macro
only supports enums with variants that have no parameters, or only have a
single name-less parameter that also implements `Action`.

An example `Action` enum might look like:

```rust
#[derive(Action, Debug)]
pub enum AllActions {
    FlushCache,
    User(UserActions)
}

#[derive(Action, Debug)]
pub enum UserActions {
    Create,
    ChangeUsername,
    Delete,
}
```

An example permissions check for `users.42` might look like:

```rust
let allowed = permissions.allowed_to(
    &ResourceName::named("users").and(42),
    &AllActions::User(UserActions::Delete)
);
```

## Permission-driven async API

At the core of many networked APIs written in Rust is an enum that represents
a request, and similarly there are usually common response/error types. In
these applications, there is usually a manually-written match statement
that, for readability and maintainability, simply pass the parameters from
the request to a helper method to handle the actual logic of the request.

The goal of the API portion of this crate is to replace the aforementioned
boilerplate match statement with a simple derive macro. For a commented example, check out [`actionable/examples/api-simulator.rs`](https://github.com/khonsulabs/actionable/blob/main/actionable/examples/api-simulator.rs).

## Open-source Licenses

This project, like all projects from [Khonsu Labs](https://khonsulabs.com/), are open-source. This repository is available under the [MIT License](./LICENSE-MIT) or the [Apache License 2.0](./LICENSE-APACHE).
