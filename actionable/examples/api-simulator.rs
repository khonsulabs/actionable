use actionable::{Action, ActionNameList, Actionable, Permissions, ResourceName, Statement};
use async_trait::async_trait;
use std::{
    io::{self, BufRead},
    sync::Arc,
};
use tokio::sync::Mutex;

/// This enum is what a "client" will send the "server" in our API.
// The derive(Actionable) macro is generating multiple traits for our "server"
// to implement:
//
// * `ApiRequestDispatcher`: Defines the Output and Error types, as well as the
//   types that implement the remaining traits.
// * `ListUsersHandler`: A trait that defines the function invoked when a `ListUsers` request is dispatched.
// * `AddUserHandler`: A trait that defines the function invoked when a `AddUser` request is dispatched.
// * `DeleteUserHandler`: A trait that defines the function invoked when a `DeleteUser` request is dispatched.
#[derive(Actionable, Debug)]
pub enum ApiRequest {
    /// An unprotected API request with no permissions checks
    #[actionable(protection = "none")]
    ListUsers,

    /// A protected API request that requires implementing an
    /// `is_allowed()` method in the handler.
    #[actionable(protection = "custom")]
    AddUser(String),

    /// A protected API request that requires implementing `resource_name(&str)
    /// -> ResourceName` and `action() -> Action` in the handler.
    #[actionable(protection = "simple")]
    DeleteUser { username: String },
}

/// This enum is what the "server" responds with to an `ApiRequest`.
pub enum ApiResponse {
    Empty,
    UserAdded,
    UserDeleted,
}

/// The actions that can be allowed or denied within this api.
#[derive(Action, Debug)]
enum ApiActions {
    AddUser,
    DeleteUser,
}

/// This type contains the state of the "server": a list of users.
struct Dispatcher {
    users: Arc<Mutex<Vec<String>>>,
}

/// This is the implementation of the dispatcher for `ApiRequest`.
impl ApiRequestDispatcher for Dispatcher {
    type Output = ApiResponse;
    type Error = anyhow::Error;

    type ListUsersHandler = Self;
    type AddUserHandler = Self;
    type DeleteUserHandler = Self;
}

/// Handles `ApiRequest::ListUsers`
#[async_trait]
impl ListUsersHandler for Dispatcher {
    type Dispatcher = Self;

    async fn handle(dispatcher: &Self, _permissions: &Permissions) -> anyhow::Result<ApiResponse> {
        let users = dispatcher.users.lock().await;
        println!("Current users:");
        for user in users.iter() {
            println!("{}", user)
        }

        Ok(ApiResponse::Empty)
    }
}

/// Handles `ApiRequest::AddUser`
#[async_trait]
impl AddUserHandler for Dispatcher {
    type Dispatcher = Self;

    async fn verify_permissions(
        _dispatcher: &Self::Dispatcher,
        permissions: &Permissions,
        username: &String,
    ) -> anyhow::Result<()> {
        // If you need just a single permissions check, you should just use the
        // "simple" protection approach. If you're needing more complicated
        // behavior, the "custom" protection allows you to do whatever
        // permission validation you need.
        if permissions.allowed_to(ResourceName::named(username), &ApiActions::AddUser) {
            Ok(())
        } else {
            anyhow::bail!("Not allowed to delete users")
        }
    }

    async fn handle_protected(
        dispatcher: &Self::Dispatcher,
        _permissions: &Permissions,
        username: String,
    ) -> anyhow::Result<ApiResponse> {
        let mut users = dispatcher.users.lock().await;
        users.push(username);
        users.sort();
        println!("User added.");
        Ok(ApiResponse::UserAdded)
    }
}

/// Handles `ApiRequest::DeleteUser`
#[async_trait]
impl DeleteUserHandler for Dispatcher {
    type Dispatcher = Self;
    type Action = ApiActions;

    fn resource_name<'a>(_dispatcher: &Self::Dispatcher, username: &'a String) -> ResourceName<'a> {
        ResourceName::named(username.clone())
    }

    fn action() -> Self::Action {
        ApiActions::DeleteUser
    }

    async fn handle_protected(
        dispatcher: &Self::Dispatcher,
        _permissions: &Permissions,
        username: String,
    ) -> anyhow::Result<ApiResponse> {
        let mut users = dispatcher.users.lock().await;
        let old_len = users.len();
        users.retain(|u| u != &username);

        if old_len != users.len() {
            println!("User removed.");
        } else {
            anyhow::bail!("User {} not found", username)
        }

        Ok(ApiResponse::UserDeleted)
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // The initial set of users known in the system.
    let users = vec![
        String::from("admin"),
        String::from("jane"),
        String::from("jon"),
        String::from("jill"),
        String::from("jim"),
    ];

    // In a real program, you should build some sort of mechanism for storing permissions, not hard-coding them like this.

    // "admin" can do anything
    let admin_permissions = Permissions::from(vec![Statement {
        resources: vec![ResourceName::any()],
        actions: ActionNameList::All,
        allowed: true,
    }]);

    // Any user that is in the list can create other users.
    let known_user_permissions = Permissions::from(vec![Statement {
        resources: vec![ResourceName::any()],
        actions: ActionNameList::from(ApiActions::AddUser),
        allowed: true,
    }]);

    // For inexplicable reasons, all unregistered users can delete jon
    let default_permissions = Permissions::from(vec![Statement {
        resources: vec![ResourceName::named("jon")],
        actions: ActionNameList::from(ApiActions::DeleteUser),
        allowed: true,
    }]);

    // Create our dispatcher, which is the "server" in this example.
    let dispatcher = Dispatcher {
        users: Arc::new(Mutex::new(users)),
    };

    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    loop {
        // Each iteration through the loop, allow the user to "log in" with their name.
        println!("Welcome to the user service. Please enter your name:");
        let name = lines.next().expect("need a name")?;

        // Determin the user's effective permissions.
        let effective_permissions = if name == "admin" {
            &admin_permissions
        } else {
            let users = dispatcher.users.lock().await;
            if users.contains(&name) {
                &known_user_permissions
            } else {
                &default_permissions
            }
        };

        // Print the main menu
        println!(
            "Hello, {}! Please enter the number of command you wish to execute:",
            name
        );
        println!("1. List Users");
        println!("2. Add User");
        println!("3. Remove User");
        println!("4. Exit");
        // Create the API request from their responses.
        let request = match lines.next().expect("no command")?.parse()? {
            1u32 => ApiRequest::ListUsers,
            2 => {
                println!("Enter the new user's name:");
                let new_user_name = lines.next().unwrap()?;
                ApiRequest::AddUser(new_user_name)
            }
            3 => {
                println!("Enter the name of the user you wish to remove:");
                let username = lines.next().unwrap()?;
                ApiRequest::DeleteUser { username }
            }
            4 => break,
            other => {
                println!("Unknown command number {}. Exiting.", other);
                continue;
            }
        };

        // Dispatch the request. The appropriate handler will be invoked.
        if let Err(err) = dispatcher.dispatch(effective_permissions, request).await {
            println!("Error received: {:?}", err);
        }
    }

    Ok(())
}
