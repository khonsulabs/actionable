//! Macros for the `actionable` API framework.

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

use proc_macro::TokenStream;
use proc_macro_error::{emit_error, proc_macro_error};
use syn::{parse_macro_input, DeriveInput};

mod action;
mod actionable;

/// Derives the `actionable::Action` trait.
#[proc_macro_error]
#[proc_macro_derive(Action)]
pub fn action_derive(input: TokenStream) -> TokenStream {
    action::derive(input)
}

/// Creates a trait named `Actionable`.
// TODO document better once done
#[proc_macro_error]
#[proc_macro_derive(Actionable, attributes(actionable))]
pub fn actionable_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match actionable::derive(&input) {
        Ok(tokens) => tokens.into(),
        Err(err) => {
            emit_error!(input.ident, err.to_string());
            TokenStream::default()
        }
    }
}
