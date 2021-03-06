#![allow(unused_variables)]

use std::borrow::Cow;

use crate::{
    Action, ActionName, Actionable, AsyncDispatcher, PermissionDenied, Permissions, ResourceName,
    Statement,
};

#[derive(Debug, Action)]
#[action(actionable = crate)]
enum TestActions {
    DoSomething,
    Post(PostActions),
}

#[derive(Debug, Action)]
#[action(actionable = crate)]
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
        Statement::for_any().allowing(&TestActions::Post(PostActions::Read)),
        // Allow all actions for the resource named all-actions-allowed
        Statement::for_resource("all-actions-allowed").allowing_all(),
        // Allow all Post actions for the resource named only-post-actions-allowed
        Statement::for_resource("only-post-actions-allowed")
            .allowing(&ActionName(vec![Cow::Borrowed("Post")])),
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

#[test]
fn multiple_actions() {
    // Default action is deny
    let statements = vec![
        // Allow Read on all.
        Statement::for_any()
            .allowing(&TestActions::Post(PostActions::Read))
            .allowing(&TestActions::Post(PostActions::Delete)),
    ];
    let permissions = Permissions::from(statements);

    // Check the positive cases:
    assert!(permissions.allowed_to(
        &ResourceName::named("someresource"),
        &TestActions::Post(PostActions::Read)
    ));
    assert!(permissions.allowed_to(
        &ResourceName::named("someresource"),
        &TestActions::Post(PostActions::Delete)
    ));

    // Check another permission
    assert!(!permissions.allowed_to(
        &ResourceName::named("someresource"),
        &TestActions::DoSomething
    ));
}

#[derive(Actionable, Debug)]
#[actionable(actionable = crate, async)]
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

#[derive(AsyncDispatcher, Debug)]
#[dispatcher(input = Request, actionable = crate)]
struct TestDispatcher;

#[async_trait::async_trait]
impl RequestDispatcher for TestDispatcher {
    type Error = TestError;
    type Output = Option<u64>;
}

#[derive(thiserror::Error, Debug)]
pub enum TestError {
    #[error("custom error")]
    CustomError,
    #[error("permission error: {0}")]
    PermissionDenied(#[from] PermissionDenied),
}

#[async_trait::async_trait]
impl UnprotectedEnumParameterHandler for TestDispatcher {
    async fn handle(
        &self,
        _permissions: &Permissions,
        arg1: u64,
    ) -> Result<Option<u64>, TestError> {
        Ok(Some(arg1))
    }
}

#[async_trait::async_trait]
impl UnprotectedStructParameterHandler for TestDispatcher {
    async fn handle(
        &self,
        _permissions: &Permissions,
        value: u64,
    ) -> Result<Option<u64>, TestError> {
        Ok(Some(value))
    }
}

#[async_trait::async_trait]
impl UnprotectedNoParametersHandler for TestDispatcher {
    async fn handle(&self, _permissions: &Permissions) -> Result<Option<u64>, TestError> {
        Ok(None)
    }
}

#[async_trait::async_trait]
impl SimplyProtectedEnumParameterHandler for TestDispatcher {
    type Action = TestActions;

    async fn resource_name<'a>(&'a self, arg1: &'a u64) -> Result<ResourceName<'a>, TestError> {
        Ok(ResourceName::named(*arg1))
    }

    fn action() -> Self::Action {
        TestActions::DoSomething
    }

    async fn handle_protected(
        &self,
        _permissions: &Permissions,
        arg1: u64,
    ) -> Result<Option<u64>, TestError> {
        Ok(Some(arg1))
    }
}

#[async_trait::async_trait]
impl SimplyProtectedStructParameterHandler for TestDispatcher {
    type Action = TestActions;

    async fn resource_name<'a>(&'a self, arg1: &'a u64) -> Result<ResourceName<'a>, TestError> {
        Ok(ResourceName::named(*arg1))
    }

    fn action() -> Self::Action {
        TestActions::DoSomething
    }

    async fn handle_protected(
        &self,
        _permissions: &Permissions,
        value: u64,
    ) -> Result<Option<u64>, TestError> {
        Ok(Some(value))
    }
}

#[async_trait::async_trait]
impl SimplyProtectedNoParametersHandler for TestDispatcher {
    type Action = TestActions;

    async fn resource_name<'a>(&'a self) -> Result<ResourceName<'a>, TestError> {
        Ok(ResourceName::named(0))
    }

    fn action() -> Self::Action {
        TestActions::DoSomething
    }

    async fn handle_protected(&self, _permissions: &Permissions) -> Result<Option<u64>, TestError> {
        Ok(None)
    }
}

#[async_trait::async_trait]
impl CustomProtectedNoParametersHandler for TestDispatcher {
    async fn verify_permissions(&self, permissions: &Permissions) -> Result<(), TestError> {
        if permissions.allowed_to(&ResourceName::named(0), &TestActions::DoSomething) {
            Ok(())
        } else {
            Err(TestError::CustomError)
        }
    }

    async fn handle_protected(&self, _permissions: &Permissions) -> Result<Option<u64>, TestError> {
        Ok(None)
    }
}

#[async_trait::async_trait]
impl CustomProtectedEnumParameterHandler for TestDispatcher {
    async fn verify_permissions(
        &self,
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
        &self,
        _permissions: &Permissions,
        arg1: u64,
    ) -> Result<Option<u64>, TestError> {
        Ok(Some(arg1))
    }
}

#[async_trait::async_trait]
impl CustomProtectedStructParameterHandler for TestDispatcher {
    async fn verify_permissions(
        &self,
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
        &self,
        _permissions: &Permissions,
        value: u64,
    ) -> Result<Option<u64>, TestError> {
        Ok(Some(value))
    }
}

#[derive(Actionable, Debug)]
#[actionable(actionable = crate, async)]
enum GenericRequest<T> {
    #[actionable(protection = "none")]
    NonGeneric,
    #[actionable(protection = "none", subaction)]
    Sub(T),
}

#[derive(AsyncDispatcher, Debug)]
#[dispatcher(
    input = GenericRequest<Request>,
    actionable = crate
)]
struct GenericDispatcher;

#[async_trait::async_trait]
impl GenericRequestDispatcher for GenericDispatcher {
    type Error = TestError;
    type Output = Option<u64>;
    type Subaction = Request;

    async fn handle_subaction(
        &self,
        permissions: &Permissions,
        subaction: Request,
    ) -> Result<Option<u64>, TestError> {
        TestDispatcher.dispatch(permissions, subaction).await
    }
}

#[async_trait::async_trait]
impl NonGenericHandler for GenericDispatcher {
    async fn handle(&self, permissions: &Permissions) -> Result<Option<u64>, TestError> {
        Ok(Some(52))
    }
}

#[tokio::test]
#[allow(clippy::too_many_lines)]
async fn example() {
    let permissions = Permissions::from(vec![Statement::for_resource(42).allowing_all()]);
    let dispatcher = TestDispatcher;

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
            .dispatch(&permissions, Request::UnprotectedStructParameter {
                value: 42
            },)
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
            .dispatch(&permissions, Request::SimplyProtectedStructParameter {
                value: 42
            },)
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
            .dispatch(&permissions, Request::CustomProtectedStructParameter {
                value: 42
            },)
            .await
            .unwrap(),
        Some(42)
    );

    // Generic dispatching
    assert!(matches!(
        GenericDispatcher
            .dispatch(&permissions, GenericRequest::NonGeneric,)
            .await,
        Ok(Some(52))
    ));
    assert!(matches!(
        GenericDispatcher
            .dispatch(
                &permissions,
                GenericRequest::Sub(Request::CustomProtectedEnumParameter(42)),
            )
            .await,
        Ok(Some(42))
    ));

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
            .dispatch(&permissions, Request::SimplyProtectedStructParameter {
                value: 1
            },)
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
            .dispatch(&permissions, Request::CustomProtectedStructParameter {
                value: 1
            },)
            .await,
        Err(TestError::CustomError)
    ));
}

#[test]
fn allowed_actions_merging_tests() {
    let permissions_a = Permissions::from(vec![
        Statement::for_resource(ResourceName::named("started_with_all")).allowing_all(),
        Statement::for_resource(ResourceName::named("started_with_some"))
            .allowing(&TestActions::DoSomething),
        Statement::for_resource(ResourceName::named("nested").and("started_with_all"))
            .allowing_all(),
        Statement::for_resource(ResourceName::named("nested").and("started_with_some"))
            .allowing(&TestActions::DoSomething),
    ]);
    let permissions_b = Permissions::from(vec![
        Statement::for_resource(ResourceName::named("started_with_none")).allowing_all(),
        Statement::for_resource(ResourceName::named("started_with_some")).allowing_all(),
        Statement::for_resource(ResourceName::named("nested").and("started_with_none"))
            .allowing(&TestActions::Post(PostActions::Read)),
        Statement::for_resource(ResourceName::named("nested").and("started_with_some"))
            .allowing(&TestActions::Post(PostActions::Read)),
    ]);

    let merged = Permissions::merged([permissions_a, permissions_b].iter());
    // For the top level, on Actions we're only testing transitioning form either
    // None/Some to All
    assert!(merged.allowed_to(
        &ResourceName::named("started_with_all"),
        &TestActions::DoSomething
    ));
    assert!(merged.allowed_to(
        &ResourceName::named("started_with_none"),
        &TestActions::DoSomething
    ));
    assert!(merged.allowed_to(
        &ResourceName::named("started_with_some"),
        &TestActions::DoSomething
    ));
    assert!(merged.allowed_to(
        &ResourceName::named("started_with_some"),
        &TestActions::Post(PostActions::Delete)
    ));
    // For the nested level, the transitions will only take permissions to a Some()
    // instead of All.
    assert!(merged.allowed_to(
        &ResourceName::named("nested").and("started_with_none"),
        &TestActions::Post(PostActions::Read)
    ));
    assert!(!merged.allowed_to(
        &ResourceName::named("nested").and("started_with_none"),
        &TestActions::DoSomething
    ));

    assert!(merged.allowed_to(
        &ResourceName::named("nested").and("started_with_some"),
        &TestActions::Post(PostActions::Read)
    ));
    assert!(merged.allowed_to(
        &ResourceName::named("nested").and("started_with_some"),
        &TestActions::DoSomething
    ));
}

#[test]
fn configuration_tests() {
    let permissions_a = Permissions::from(vec![
        Statement::for_resource(ResourceName::named("a")).with("u64", 0_u64),
        Statement::for_resource(ResourceName::named("a")).with("i64", i64::MIN),
        Statement::for_resource(ResourceName::named("a").and("b")).with("i64", 2_i64),
        Statement::for_any().with("global", "value"),
    ]);

    assert_eq!(
        permissions_a
            .get(&ResourceName::named("a"), "u64")
            .unwrap()
            .to_unsigned(),
        Some(0)
    );

    assert_eq!(
        permissions_a
            .get(&ResourceName::named("a"), "u64")
            .unwrap()
            .to_signed(),
        Some(0)
    );

    assert_eq!(
        permissions_a
            .get(&ResourceName::named("a"), "u64")
            .unwrap()
            .to_string(),
        "0"
    );

    assert_eq!(
        permissions_a
            .get(&ResourceName::named("a"), "i64")
            .unwrap()
            .to_unsigned(),
        None
    );

    assert_eq!(
        permissions_a
            .get(&ResourceName::named("a"), "i64")
            .unwrap()
            .to_signed(),
        Some(i64::MIN)
    );

    assert_eq!(
        permissions_a
            .get(&ResourceName::named("a").and("b"), "i64")
            .unwrap()
            .to_signed(),
        Some(2)
    );

    assert_eq!(
        permissions_a
            .get(&ResourceName::named("a").and("b"), "global")
            .unwrap()
            .to_string(),
        "value"
    );

    let permissions_b = Permissions::from(vec![
        Statement::for_resource(ResourceName::named("a")).with("newkey", "newvalue"),
        Statement::for_any().with("global", "value2"),
    ]);
    let merged = Permissions::merged([&permissions_a, &permissions_b]);

    assert_eq!(
        merged
            .get(&ResourceName::named("a").and("b"), "global")
            .unwrap()
            .to_string(),
        "value"
    );

    assert_eq!(
        merged
            .get(&ResourceName::named("a").and("b"), "newkey")
            .unwrap()
            .to_string(),
        "newvalue"
    );
}
