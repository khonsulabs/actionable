//! An enum-based async framework for building permission-driven APIs

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
#[derive(thiserror::Error, Debug)]
#[error("Action '{action}' was denied on resource'{resource}'")]
pub struct PermissionDenied {
    /// The resource that `action` was attempted upon.
    pub resource: ResourceName,
    /// The `action` attempted upon `resource`.
    pub action: ActionName,
}
