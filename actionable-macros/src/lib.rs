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
mod dispatcher;

/// Derives the `actionable::Action` trait.
///
/// This trait can be customizd using the `action` attribute in these ways:
///
/// * Crate name override: `#[action(actionable = "someothername")]`. If you
///   find yourself needing to import `actionable` as another name, this setting
///   will replace all mentions of `actionable` with the identifier specified.
#[proc_macro_error]
#[proc_macro_derive(Action, attributes(action))]
pub fn action_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match action::derive(&input) {
        Ok(tokens) => tokens.into(),
        Err(err) => {
            emit_error!(input.ident, err.to_string());
            TokenStream::default()
        }
    }
}

/// Derives a set of traits that can be used to implement a permissions-driven
/// API. There are options that can be customized with the `#[actionable]`
/// attribute at the enum level:
///
/// * Crate name override: `#[actionable(actionable = "someothername")]`. If you
///   find yourself needing to import `actionable` as another name, this setting
///   will replace all mentions of `actionable` with the identifier specified.
///
/// ## The Dispatcher Trait
///
/// The first trait that is generated is named `<EnumName>Dispatcher`. For
/// example, if the enum's name is `Request`, the generated trait name will be
/// `RequestDispatcher`. This trait has no methods for you to implement. It
/// defines several associated types:
///
/// * `Output`: The `Ok` side of the `Result`.
/// * `Error`: The `Err` side of the `Result`. Must implement
///   `From<actionable::PermissionDenied>`.
/// * For each variant in the enum, another associated type named
///   `<VariantName>Handler`. For example, if the enum variant was
///   `Request::AddUser`, the associated type will be `AddUserHandler`. Each of
///   these associated types must implement the trait of the same name
///   (described in the next section).
///
/// The dispatcher trait has a method available for you to use to dispatch
/// requests: `async fn dispatch(&self, permissions: &Permissions, request:
/// <EnumName>) -> Result<Self::Output, Self::Error>`.
///
/// ## The Handler Traits
///
/// For each variant in the enum, a trait will be generated named
/// `<VariantName>Handler`. Using the same example above, the enum variant
/// `Request::AddUser` would generate the trait `AddUserHandler`. These traits
/// are implemented using the
/// [`async-trait`](https://crates.io/crate/async-trait) trait.
///
/// Each variant must have a protection method assigned using the
/// `#[actionable]` attribute. There are three protection methods:
///
/// ### No Protection: `#[actionable(protection = "none")]`
///
/// A handler with no protection has one method:
///
/// ```rust
/// # type Output = ();
/// # type Error = ();
/// # use actionable::{Permissions, async_trait};
/// #[async_trait]
/// trait Handler {
///    type Dispatcher;
///    async fn handle(
///        dispatcher: &Self::Dispatcher,
///        permissions: &Permissions,
///        /* each field on this variant is passed
///           as a parameter to this method */
///    ) -> Result<Output, Error>;
/// }
/// ```
///
/// Actionable does not do any checks before invoking this handler.
///
/// ### Simple Protection: `#[actionable(protection = "simple")]`
///
/// A handler with simple protection exposes methods and types to allow
/// specifying an `actionable::ResourceName` and an `Action` for this handler:
///
/// ```rust
/// # type Output = ();
/// # type Error = ();
/// # use actionable::{Permissions, ResourceName, async_trait};
/// #[async_trait]
/// trait Handler {
///    type Dispatcher;
///    type Action;
///
///    fn resource_name<'a>(
///        dispatcher: &Self::Dispatcher,
///        /* each field on this variant is passed
///           by reference as a parameter to this method */
///    ) -> ResourceName<'a>;
///
///    fn action() -> Self::Action;
///
///    async fn handle_protected(
///        dispatcher: &Self::Dispatcher,
///        permissions: &Permissions,
///        /* each field on this variant is passed
///           as a parameter to this method */
///    ) -> Result<Output, Error>;
/// }
/// ```
///
/// When the handler is invoked, it first checks `permissions` to ensure that
/// `action()` is allowed to be performed on `resource_name()`. If it is not
/// allowed, an `actionable::PermissionDenied` error will be returned. If it is
/// allowed, `handle_protected()` will be executed.
///
/// ### Custom Protection: `#[actionable(protection = "custom")]`
///
/// A handler with custom protection has two methods, one to verify permissions
/// and one to execute the protected code:
///
/// ```rust
/// # type Output = ();
/// # type Error = ();
/// # use actionable::{Permissions, async_trait};
/// #[async_trait]
/// trait Handler {
///    type Dispatcher;
///    async fn verify_permissions(
///        dispatcher: &Self::Dispatcher,
///        permissions: &Permissions,
///        /* each field on this variant is passed
///           by refrence as a parameter to this method */
///    ) -> Result<(), Error>;
///
///    async fn handle_protected(
///        dispatcher: &Self::Dispatcher,
///        permissions: &Permissions,
///        /* each field on this variant is passed as a parameter
///        to this method */
///    ) -> Result<Output, Error>;
/// }
/// ```
///
/// Actionable will first call `verify_permissions()`. If you return `Ok(())`,
/// your `handle_protected()` method is invoked.
///
/// ## Why should you use the built-in protection modes?
///
/// Actionable attempts to make permission handling easy to understand and
/// implement while making it difficult to forget implementing permission
/// handling. This is only effective if you use the protection levels.
///
/// Because Actionable includes `permissions` in every call to
/// `handle[_protected]()`, technically you could use a protection level of
/// `none` and implement permission handling within the `handle()` function.
/// While it would work, you shouldn't do this.
///
/// Actionable encourages placing information about permission handling in the
/// definition of the enum. By using `simple` and `custom` protection
/// strategies, consumers of your API will be able to see at the enum level what
/// APIs check permissions. When trying to understand what permissions are being
/// used, this is critical.
///
/// By placing your permission handling code in locations that follow a
/// repeatable patern, you're helping anyone who is reading the code separate
/// what logic is related to permission handling and what logic is related to
/// the API implementation.
///
/// ## What protection mode should you use?
///
/// * If your handler is operating on a single resource and performing a single
///   action, use the `simple` protection mode.
/// * If your handler needs to check permissions but it's more complicated than
///   the first scenario, use the `custom` protection mode.
/// * If you aren't enforcing permissions inside of this handler, use the `none`
///   protection mode.
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

/// Derives the `Dispatcher` trait.
///
/// This trait requires the `input` parameter to be specified. The full list of
/// parameters that can be customized are:
///
/// * `input` Type: `#[dispatcher(input = "EnumName")]`. The enum name here
///   needs to have had [`Actionable`] derived on it.
/// * Crate name override: `#[actionable(actionable = "someothername")]`. If you
///   find yourself needing to import `actionable` as another name, this setting
///   will replace all mentions of `actionable` with the identifier specified.
///
/// The `input` type must be in scope, as do the derived traits generated by
/// deriving `Actionable`.
#[proc_macro_error]
#[proc_macro_derive(Dispatcher, attributes(dispatcher))]
pub fn dispatcher_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match dispatcher::derive(&input) {
        Ok(tokens) => tokens.into(),
        Err(err) => {
            emit_error!(input.ident, err.to_string());
            TokenStream::default()
        }
    }
}
