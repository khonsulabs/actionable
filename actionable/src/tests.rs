#![allow(unused_variables)]

use std::borrow::Cow;

use crate::{
    Action, ActionName, ActionNameList, Actionable, PermissionDenied, Permissions, ResourceName,
    Statement,
};

use crate as actionable; // TODO the Action derive doesn't accept this customization

#[derive(Debug, Action)]
enum TestActions {
    DoSomething,
    Post(PostActions),
}

#[derive(Debug, Action)]
enum PostActions {
    Read,
    Update,
    Delete,
}

#[test]
fn basics() {
    // Default action is deny
    let statements = vec![
        // Allow Read on all.
        Statement {
            resources: vec![ResourceName::any()],
            actions: ActionNameList::from(TestActions::Post(PostActions::Read)),
            allowed: true,
        },
        // Allow all actions for the resource named all-actions-allowed
        Statement {
            resources: vec![ResourceName::named("all-actions-allowed")],
            actions: ActionNameList::All,
            allowed: true,
        },
        // Allow all Post actions for the resource named only-post-actions-allowed
        Statement {
            resources: vec![ResourceName::named("only-post-actions-allowed")],
            actions: ActionNameList::from(ActionName(vec![Cow::Borrowed("Post")])),
            allowed: true,
        },
    ];
    let permissions = Permissions::from(statements);

    // Check the positive cases:
    assert!(permissions.allowed_to(
        &ResourceName::named("someresource"),
        &TestActions::Post(PostActions::Read)
    ));
    assert!(permissions.allowed_to(
        &ResourceName::named("all-actions-allowed"),
        &TestActions::Post(PostActions::Update)
    ));
    assert!(permissions.allowed_to(
        &ResourceName::named("all-actions-allowed"),
        &TestActions::DoSomething
    ));
    assert!(permissions.allowed_to(
        &ResourceName::named("only-post-actions-allowed"),
        &TestActions::Post(PostActions::Delete)
    ));

    // Test the negatives
    assert!(!permissions.allowed_to(
        &ResourceName::named("someresource"),
        &TestActions::Post(PostActions::Update)
    ));
    assert!(!permissions.allowed_to(
        &ResourceName::named("someresource"),
        &TestActions::Post(PostActions::Delete)
    ));
    assert!(!permissions.allowed_to(
        &ResourceName::named("only-post-actions-allowed"),
        &TestActions::DoSomething
    ));
}

#[derive(Actionable, Debug)]
#[actionable(actionable = "crate")]
enum Request {
    #[actionable(protection = "none")]
    UnprotectedNoParameters,
    #[actionable(protection = "none")]
    UnprotectedEnumParameter(u64),
    #[actionable(protection = "none")]
    UnprotectedStructParameter { value: u64 },

    #[actionable(protection = "simple")]
    SimplyProtectedNoParameters,
    #[actionable(protection = "simple")]
    SimplyProtectedEnumParameter(u64),
    #[actionable(protection = "simple")]
    SimplyProtectedStructParameter { value: u64 },

    #[actionable(protection = "custom")]
    CustomProtectedNoParameters,
    #[actionable(protection = "custom")]
    CustomProtectedEnumParameter(u64),
    #[actionable(protection = "custom")]
    CustomProtectedStructParameter { value: u64 },
}

struct Dispatcher;

#[async_trait::async_trait]
impl RequestDispatcher for Dispatcher {
    type Output = Option<u64>;
    type Error = TestError;

    type UnprotectedNoParametersHandler = Self;
    type UnprotectedEnumParameterHandler = Self;
    type UnprotectedStructParameterHandler = Self;

    type SimplyProtectedNoParametersHandler = Self;
    type SimplyProtectedEnumParameterHandler = Self;
    type SimplyProtectedStructParameterHandler = Self;

    type CustomProtectedNoParametersHandler = Self;
    type CustomProtectedEnumParameterHandler = Self;
    type CustomProtectedStructParameterHandler = Self;
}

#[derive(thiserror::Error, Debug)]
pub enum TestError {
    #[error("custom error")]
    CustomError,
    #[error("permission error: {0}")]
    PermissionDenied(#[from] PermissionDenied),
}

#[async_trait::async_trait]
impl UnprotectedEnumParameterHandler for Dispatcher {
    type Dispatcher = Self;

    async fn handle(
        dispatcher: &Self::Dispatcher,
        _permissions: &Permissions,
        arg1: u64,
    ) -> Result<Option<u64>, TestError> {
        Ok(Some(arg1))
    }
}

#[async_trait::async_trait]
impl UnprotectedStructParameterHandler for Dispatcher {
    type Dispatcher = Self;

    async fn handle(
        dispatcher: &Self::Dispatcher,
        _permissions: &Permissions,
        value: u64,
    ) -> Result<Option<u64>, TestError> {
        Ok(Some(value))
    }
}

#[async_trait::async_trait]
impl UnprotectedNoParametersHandler for Dispatcher {
    type Dispatcher = Self;

    async fn handle(
        dispatcher: &Self::Dispatcher,
        _permissions: &Permissions,
    ) -> Result<Option<u64>, TestError> {
        Ok(None)
    }
}

#[async_trait::async_trait]
impl SimplyProtectedEnumParameterHandler for Dispatcher {
    type Dispatcher = Self;
    type Action = TestActions;

    fn resource_name<'a>(_dispatcher: &Self::Dispatcher, arg1: &'a u64) -> ResourceName<'a> {
        ResourceName::named(*arg1)
    }

    fn action() -> Self::Action {
        TestActions::DoSomething
    }

    async fn handle_protected(
        dispatcher: &Self::Dispatcher,
        _permissions: &Permissions,
        arg1: u64,
    ) -> Result<Option<u64>, TestError> {
        Ok(Some(arg1))
    }
}

#[async_trait::async_trait]
impl SimplyProtectedStructParameterHandler for Dispatcher {
    type Dispatcher = Self;
    type Action = TestActions;

    fn resource_name<'a>(_dispatcher: &Self::Dispatcher, arg1: &'a u64) -> ResourceName<'a> {
        ResourceName::named(*arg1)
    }

    fn action() -> Self::Action {
        TestActions::DoSomething
    }

    async fn handle_protected(
        dispatcher: &Self::Dispatcher,
        _permissions: &Permissions,
        value: u64,
    ) -> Result<Option<u64>, TestError> {
        Ok(Some(value))
    }
}

#[async_trait::async_trait]
impl SimplyProtectedNoParametersHandler for Dispatcher {
    type Dispatcher = Self;
    type Action = TestActions;

    fn resource_name(_dispatcher: &Self::Dispatcher) -> ResourceName<'static> {
        ResourceName::named(0)
    }

    fn action() -> Self::Action {
        TestActions::DoSomething
    }

    async fn handle_protected(
        dispatcher: &Self::Dispatcher,
        _permissions: &Permissions,
    ) -> Result<Option<u64>, TestError> {
        Ok(None)
    }
}

#[async_trait::async_trait]
impl CustomProtectedNoParametersHandler for Dispatcher {
    type Dispatcher = Self;

    async fn verify_permissions(
        _dispatcher: &Self,
        permissions: &Permissions,
    ) -> Result<(), TestError> {
        if permissions.allowed_to(&ResourceName::named(0), &TestActions::DoSomething) {
            Ok(())
        } else {
            Err(TestError::CustomError)
        }
    }

    async fn handle_protected(dispatcher: &Self::Dispatcher) -> Result<Option<u64>, TestError> {
        Ok(None)
    }
}

#[async_trait::async_trait]
impl CustomProtectedEnumParameterHandler for Dispatcher {
    type Dispatcher = Self;

    async fn verify_permissions(
        _dispatcher: &Self,
        permissions: &Permissions,
        arg1: &u64,
    ) -> Result<(), TestError> {
        if permissions.allowed_to(&ResourceName::named(*arg1), &TestActions::DoSomething) {
            Ok(())
        } else {
            Err(TestError::CustomError)
        }
    }

    async fn handle_protected(
        dispatcher: &Self::Dispatcher,
        arg1: u64,
    ) -> Result<Option<u64>, TestError> {
        Ok(Some(arg1))
    }
}

#[async_trait::async_trait]
impl CustomProtectedStructParameterHandler for Dispatcher {
    type Dispatcher = Self;

    async fn verify_permissions(
        _dispatcher: &Self,
        permissions: &Permissions,
        arg1: &u64,
    ) -> Result<(), TestError> {
        if permissions.allowed_to(&ResourceName::named(*arg1), &TestActions::DoSomething) {
            Ok(())
        } else {
            Err(TestError::CustomError)
        }
    }

    async fn handle_protected(
        dispatcher: &Self::Dispatcher,
        value: u64,
    ) -> Result<Option<u64>, TestError> {
        Ok(Some(value))
    }
}

#[tokio::test]
async fn example() {
    let permissions = Permissions::from(vec![Statement {
        resources: vec![ResourceName::named(42)],
        actions: ActionNameList::All,
        allowed: true,
    }]);
    let dispatcher = Dispatcher;

    // All success (permitted) cases
    assert_eq!(
        dispatcher
            .dispatch(&permissions, Request::UnprotectedEnumParameter(42),)
            .await
            .unwrap(),
        Some(42)
    );
    assert_eq!(
        dispatcher
            .dispatch(
                &permissions,
                Request::UnprotectedStructParameter { value: 42 },
            )
            .await
            .unwrap(),
        Some(42)
    );
    assert_eq!(
        dispatcher
            .dispatch(&permissions, Request::UnprotectedNoParameters,)
            .await
            .unwrap(),
        None
    );
    assert_eq!(
        dispatcher
            .dispatch(&permissions, Request::SimplyProtectedEnumParameter(42))
            .await
            .unwrap(),
        Some(42)
    );
    assert_eq!(
        dispatcher
            .dispatch(
                &permissions,
                Request::SimplyProtectedStructParameter { value: 42 },
            )
            .await
            .unwrap(),
        Some(42)
    );
    assert_eq!(
        dispatcher
            .dispatch(&permissions, Request::CustomProtectedEnumParameter(42))
            .await
            .unwrap(),
        Some(42)
    );
    assert_eq!(
        dispatcher
            .dispatch(
                &permissions,
                Request::CustomProtectedStructParameter { value: 42 },
            )
            .await
            .unwrap(),
        Some(42)
    );

    // Permission denied errors
    assert!(matches!(
        dispatcher
            .dispatch(&permissions, Request::SimplyProtectedNoParameters)
            .await,
        Err(TestError::PermissionDenied(_))
    ));
    assert!(matches!(
        dispatcher
            .dispatch(&permissions, Request::SimplyProtectedEnumParameter(1))
            .await,
        Err(TestError::PermissionDenied(_))
    ));
    assert!(matches!(
        dispatcher
            .dispatch(
                &permissions,
                Request::SimplyProtectedStructParameter { value: 1 },
            )
            .await,
        Err(TestError::PermissionDenied(_))
    ));

    // Custom errors
    assert!(matches!(
        dispatcher
            .dispatch(&permissions, Request::CustomProtectedNoParameters)
            .await,
        Err(TestError::CustomError)
    ));
    assert!(matches!(
        dispatcher
            .dispatch(&permissions, Request::CustomProtectedEnumParameter(1))
            .await,
        Err(TestError::CustomError)
    ));
    assert!(matches!(
        dispatcher
            .dispatch(
                &permissions,
                Request::CustomProtectedStructParameter { value: 1 },
            )
            .await,
        Err(TestError::CustomError)
    ));
}