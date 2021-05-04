use std::collections::HashMap;

use crate::{Action, ActionNameList, Identifier, Statement};

/// A collection of allowed permissions.
#[derive(Default, Debug)]
pub struct Permissions {
    children: Option<HashMap<Identifier<'static>, Permissions>>,
    allowed: AllowedActions,
}

impl Permissions {
    /// Evaluate whether the `action` is allowed to be taken upon
    /// `resource_name`. Returns true if the action should be allowed. If no
    /// statements that match `resource_name` allow `action`, false will be
    /// returned.
    pub fn allowed_to<R: AsRef<[Identifier<'static>]>, P: Action>(
        &self,
        resource_name: R,
        action: &P,
    ) -> bool {
        let resource_name = resource_name.as_ref();
        // This function checks all possible matches of `resource_name` by using
        // recursion to call itself for each entry in `resource_name`. This
        // first block does the function call recursion. The second block checks
        // `action`.
        if let Some(resource) = resource_name.first() {
            if let Some(children) = &self.children {
                let remaining_resource = &resource_name[1..resource_name.len()];
                // Check if there are entries for this resource segment.
                if let Some(permissions) = children.get(resource) {
                    println!("Checking allowed in {:?}", resource);
                    if permissions.allowed_to(remaining_resource, action) {
                        return dbg!(true);
                    }
                    println!("Nope");
                }

                // Check if there are entries for `Any`.
                if let Some(permissions) = children.get(&Identifier::Any) {
                    println!("Checking allowed in Any");
                    if permissions.allowed_to(remaining_resource, action) {
                        return dbg!(true);
                    }
                    println!("Nope");
                }
            }
        }

        // When execution reaches here, either resource_name is empty, or none
        // of the previous paths have reached an "allow" state. The purpose of
        // this chunk of code is to determine if this action is allowed based on
        // this node's list of approved actions. This is also evaluated
        // recursively, but at any stage if we reach match (positive or
        // negative), we we can return.
        let mut allowed = &self.allowed;
        for name in action.name().0 {
            println!("checking permissions for {:?} in {:?}", name, allowed);
            allowed = match allowed {
                AllowedActions::None => return dbg!(false),
                AllowedActions::All => return dbg!(true),
                AllowedActions::Some(actions) => {
                    if let Some(children_allowed) = actions.get(name.as_ref()) {
                        children_allowed
                    } else {
                        println!("Didn't find child: {:?}", name);
                        return false;
                    }
                }
            };
        }
        dbg!(matches!(allowed, AllowedActions::All))
    }
}

impl From<Vec<Statement>> for Permissions {
    fn from(statements: Vec<Statement>) -> Self {
        let mut permissions = Self::default();
        for statement in statements {
            // Apply this statement to all resources
            for resource in statement.resources {
                let mut current_permissions = &mut permissions;
                // Look up the permissions for the resource path
                for name in resource {
                    let permissions = current_permissions
                        .children
                        .get_or_insert_with(HashMap::default);
                    current_permissions = permissions.entry(name).or_default();
                }

                // Apply the "allowed" status to each action in this resource.
                let mut allowed = &mut current_permissions.allowed;
                match &statement.actions {
                    ActionNameList::List(actions) => {
                        for action in actions {
                            for name in &action.0 {
                                let action_map = match allowed {
                                    AllowedActions::All | AllowedActions::None => {
                                        *allowed = {
                                            let mut action_map = HashMap::new();
                                            action_map
                                                .insert(name.to_string(), AllowedActions::None);
                                            AllowedActions::Some(action_map)
                                        };
                                        if let AllowedActions::Some(action_map) = allowed {
                                            action_map
                                        } else {
                                            unreachable!()
                                        }
                                    }
                                    AllowedActions::Some(action_map) => action_map,
                                };
                                allowed = action_map.entry(name.to_string()).or_default();
                            }
                        }
                    }
                    ActionNameList::All => {}
                }

                if statement.allowed {
                    *allowed = AllowedActions::All
                } else {
                    *allowed = AllowedActions::None
                }
            }
        }
        permissions
    }
}

#[derive(Debug)]
enum AllowedActions {
    None,
    Some(HashMap<String, AllowedActions>),
    All,
}

impl Default for AllowedActions {
    fn default() -> Self {
        Self::None
    }
}
