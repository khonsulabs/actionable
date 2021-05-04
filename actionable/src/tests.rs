use std::borrow::Cow;

use crate::{Action, ActionName, ActionNameList, Actionable, Permissions, ResourceName, Statement};

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

use crate as actionable;

#[derive(Actionable, Debug)]
enum Request {
    UnprotectedEnumParameter(u64),
    UnprotectedNoParameters,
    #[actionable(protection = "simple")]
    SimplyPotectedEnumParameter(u64),
    #[actionable(protection = "simple")]
    SimplyProtectedNoParameters,
    #[actionable(protection = "custom")]
    CustomProtectedEnumParameter(u64),
    #[actionable(protection = "custom")]
    CustomProtectedNoParameters,
    // #[actionable(subaction)]
    // Custom(YourType)
}

// #[actionable(parent = Request)]
// enum YourType {

// }

struct Dispatcher;

#[async_trait::async_trait]
impl RequestDispatcher for Dispatcher {
    type Output = ();
    type Error = anyhow::Error;

    type UnprotectedEnumParameterHandler = Self;
    type UnprotectedNoParametersHandler = Self;
    type SimplyPotectedEnumParameterHandler = Self;
    type SimplyProtectedNoParametersHandler = Self;
    type CustomProtectedNoParametersHandler = Self;
    type CustomProtectedEnumParameterHandler = Self;
}

#[async_trait::async_trait]
impl UnprotectedEnumParameterHandler for Dispatcher {
    type Dispatcher = Self;

    async fn handle(dispatcher: &Self::Dispatcher, arg1: u64) -> Result<(), anyhow::Error> {
        todo!()
    }
}

#[async_trait::async_trait]
impl UnprotectedNoParametersHandler for Dispatcher {
    type Dispatcher = Self;

    async fn handle(dispatcher: &Self::Dispatcher) -> Result<(), anyhow::Error> {
        todo!()
    }
}

#[async_trait::async_trait]
impl SimplyPotectedEnumParameterHandler for Dispatcher {
    type Dispatcher = Self;
    type Action = TestActions;

    fn resource_name(arg1: &u64) -> ResourceName {
        todo!()
    }

    fn action() -> Self::Action {
        TestActions::DoSomething
    }

    async fn handle_protected(
        dispatcher: &Self::Dispatcher,
        arg1: u64,
    ) -> Result<(), anyhow::Error> {
        todo!()
    }
}

#[async_trait::async_trait]
impl SimplyProtectedNoParametersHandler for Dispatcher {
    type Dispatcher = Self;
    type Action = TestActions;

    fn resource_name() -> ResourceName {
        todo!()
    }

    fn action() -> Self::Action {
        TestActions::DoSomething
    }

    async fn handle_protected(dispatcher: &Self::Dispatcher) -> Result<(), anyhow::Error> {
        todo!()
    }
}

#[async_trait::async_trait]
impl CustomProtectedNoParametersHandler for Dispatcher {
    type Dispatcher = Self;

    fn is_allowed(permissions: &Permissions) -> bool {
        todo!()
    }

    async fn handle_protected(dispatcher: &Self::Dispatcher) -> Result<(), anyhow::Error> {
        todo!()
    }
}

#[async_trait::async_trait]
impl CustomProtectedEnumParameterHandler for Dispatcher {
    type Dispatcher = Self;

    fn is_allowed(permissions: &Permissions, arg1: &u64) -> bool {
        todo!()
    }

    async fn handle_protected(
        dispatcher: &Self::Dispatcher,
        arg1: u64,
    ) -> Result<(), anyhow::Error> {
        todo!()
    }
}

async fn example() {
    let dispatcher = Dispatcher;
    dispatcher
        .act(
            Request::SimplyPotectedEnumParameter(1),
            &Permissions::default(),
        )
        .await
        .unwrap()
}
