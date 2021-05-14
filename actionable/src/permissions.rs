use std::collections::HashMap;

use crate::{Action, ActionNameList, Identifier, Statement};

/// A collection of allowed permissions. This is constructed from a
/// `Vec<`[`Statement`]`>`. By default, no actions are allowed on any resources.
#[derive(Default, Debug)]
pub struct Permissions {
    children: Option<HashMap<Identifier<'static>, Permissions>>,
    allowed: AllowedActions,
}

impl Permissions {
    /// Returns a `Permisions` instance constructed with
    /// [`Statement::allow_all()`].
    #[must_use]
    pub fn allow_all() -> Self {
        Self::from(vec![Statement::allow_all()])
    }

    /// Evaluate whether the `action` is allowed to be taken upon
    /// `resource_name`. Returns true if the action should be allowed. If no
    /// statements that match `resource_name` allow `action`, false will be
    /// returned.
    pub fn allowed_to<'a, R: AsRef<[Identifier<'a>]>, P: Action>(
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
                    if permissions.allowed_to(remaining_resource, action) {
                        return true;
                    }
                }

                // Check if there are entries for `Any`.
                if let Some(permissions) = children.get(&Identifier::Any) {
                    if permissions.allowed_to(remaining_resource, action) {
                        return true;
                    }
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
            allowed = match allowed {
                AllowedActions::None => return false,
                AllowedActions::All => return true,
                AllowedActions::Some(actions) => {
                    if let Some(children_allowed) = actions.get(name.as_ref()) {
                        children_allowed
                    } else {
                        return false;
                    }
                }
            };
        }
        matches!(allowed, AllowedActions::All)
    }

    /// Returns a new instance that merges all allowed actions from
    /// `permissions`.
    #[must_use]
    pub fn merged(permissions: &[Self]) -> Self {
        let mut combined = Self::default();
        for incoming in permissions {
            combined.add_permissions(incoming);
        }
        combined
    }

    fn add_permissions(&mut self, permissions: &Self) {
        if let Some(children) = &permissions.children {
            let our_children = self.children.get_or_insert_with(HashMap::new);
            for (name, permissions) in children {
                let our_permissions = our_children.entry(name.clone()).or_default();
                our_permissions.add_permissions(permissions);
            }
        }

        self.allowed.add_allowed(&permissions.allowed);
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
                    ActionNameList::List(actions) =>
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
                        },
                    ActionNameList::All => {}
                }

                *allowed = AllowedActions::All
            }
        }
        permissions
    }
}

#[derive(Debug, Clone)]
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

impl AllowedActions {
    fn add_allowed(&mut self, other: &Self) {
        match other {
            Self::None => {}
            Self::Some(actions) =>
                if !matches!(self, Self::All) {
                    if let Self::Some(our_allowed) = self {
                        for (name, allowed) in actions {
                            let our_entry = our_allowed.entry(name.clone()).or_default();
                            our_entry.add_allowed(allowed);
                        }
                    } else {
                        *self = Self::Some(actions.clone());
                    }
                },
            Self::All => {
                *self = Self::All;
            }
        }
    }
}
