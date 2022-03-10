pub use actionable_macros::{AsyncDispatcher, Dispatcher};
use async_trait::async_trait;

use crate::Permissions;

/// Dispatches `T` to an appropriate handler. This trait is derivable.
pub trait Dispatcher<T>: Send + Sync {
    /// The type of the result.
    type Result: Send + Sync;

    /// Dispatches `request` to the appropriate handler while also ensuring
    /// `permissions` allows the request.
    fn dispatch(&self, permissions: &Permissions, request: T) -> Self::Result;
}

/// Dispatches `T` to an appropriate handler. This trait is derivable.
#[async_trait]
pub trait AsyncDispatcher<T>: Send + Sync {
    /// The type of the result.
    type Result: Send + Sync;

    /// Dispatches `request` to the appropriate handler while also ensuring
    /// `permissions` allows the request.
    async fn dispatch(&self, permissions: &Permissions, request: T) -> Self::Result;
}
