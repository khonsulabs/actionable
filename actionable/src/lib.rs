//! Actionable provides the basic functionality needed to build an async-based
//! API that has a flexible permissions system integrated.
//!
//! This crate was designed to be used by [`PliantDb`](https://pliantdb.dev/)
//! internally, and as a way for users of `PliantDb` to extend their database
//! servers with their own APIs.
//!
//! ## Permissions
//!
//! The [`Permissions`] struct is constructed from a list of `Statement`s. The
//! `Statement` struct is inspired by [statements in
//! IAM](https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies_elements_statement.html).
//! By default, all actions are denied for all resources.
//!
//! The [`ResourceName`] struct describes a unique name/id of *anything* in your
//! application. This is meant to be similar to [ARNs in
//! IAM](https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_identifiers.html#identifiers-arns),
//! but instead of being restricted to a format by this library, you are able to
//! define your own syntax.
//!
//! The [`Action`] trait is derive-able, and will convert any enum to something
//! that can be permitted or denied to any [`ResourceName`]. This derive macro
//! only supports enums with variants that have no parameters, or only have a
//! single name-less parameter that also implements [`Action`].
//!
//! An example [`Action`] enum might look like:
//!
//! ```rust
//! # use actionable::Action;
//! #[derive(Action, Debug)]
//! pub enum AllActions {
//!     FlushCache,
//!     User(UserActions)
//! }
//!
//! #[derive(Action, Debug)]
//! pub enum UserActions {
//!     Create,
//!     ChangeUsername,
//!     Delete,
//! }
//! ```
//!
//! An example permissions check for `users.42` might look like:
//!
//! ```rust
//! # use actionable::{Action, Permissions, ResourceName};
//! # #[derive(Action, Debug)]
//! # pub enum AllActions {
//! #     FlushCache,
//! #     User(UserActions)
//! # }
//! #
//! # #[derive(Action, Debug)]
//! # pub enum UserActions {
//! #     Create,
//! #     ChangeUsername,
//! #     Delete,
//! # }
//! # let permissions = Permissions::default();
//! let allowed = permissions.allowed_to(
//!     &ResourceName::named("users").and(42),
//!     &AllActions::User(UserActions::Delete)
//! );
//! ```
//!
//! ## Permission-driven async API
//!
//! At the core of many networked APIs written in Rust is an enum that represents
//! a request, and similarly there are usually common response/error types. In
//! these applications, there is usually a manually-written match statement
//! that, for readability and maintainability, simply pass the parameters from
//! the request to a helper method to handle the actual logic of the request.
//!
//! The goal of the API portion of this crate is to replace the aforementioned
//! boilerplate match statement with a simple derive macro. For a commented example, check out [`actionable/examples/api-simulator.rs`](https://github.com/khonsulabs/actionable/blob/main/actionable/examples/api-simulator.rs).

#![forbid(unsafe_code)]
#![warn(
    clippy::cargo,
    missing_docs,
    clippy::nursery,
    clippy::pedantic,
    future_incompatible,
    rust_2018_idioms
)]
#![cfg_attr(doc, deny(rustdoc))]

mod action;
mod permissions;
mod statement;

use serde::{Deserialize, Serialize};

pub use self::{
    action::{Action, ActionName},
    permissions::Permissions,
    statement::{ActionNameList, Identifier, ResourceName, Statement},
};

pub use actionable_macros::Actionable;
#[doc(hidden)]
pub use async_trait;

#[cfg(test)]
mod tests;

/// An `action` was denied.
#[derive(thiserror::Error, Clone, Debug, Serialize, Deserialize)]
#[error("Action '{action}' was denied on resource'{resource}'")]
pub struct PermissionDenied {
    /// The resource that `action` was attempted upon.
    pub resource: ResourceName<'static>,
    /// The `action` attempted upon `resource`.
    pub action: ActionName,
}