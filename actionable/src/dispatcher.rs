use async_trait::async_trait;

pub use actionable_macros::Dispatcher;

use crate::Permissions;

/// Dispatches `T` to an appropriate handler. This trait is derivable.
#[async_trait]
pub trait Dispatcher<T>: Send + Sync {
    /// The type of the result.
    type Result: Send + Sync;

    /// Dispatches `request` to the appropriate handler while also ensuring `permissions` allows the request.
    async fn dispatch(&self, permissions: &Permissions, request: T) -> Self::Result;
}
